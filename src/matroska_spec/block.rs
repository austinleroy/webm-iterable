use std::convert::{TryFrom, TryInto};

use ebml_iterable::tools::{self as ebml_tools, Vint};

use super::super::errors::WebmCoercionError;
use super::MatroskaSpec;

///
/// An enum describing different block lacing options.
///
/// This enum is based on the definition for [Lacing](https://www.matroska.org/technical/basics.html#lacing) as defined by the [Matroska Spec](http://www.matroska.org/technical/specs/index.html).
///
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BlockLacing {
    Xiph,
    Ebml,
    FixedSize,
}

///
/// A single frame of data within a block.
/// 
/// There may be a single frame or multiple frames within a "Block" or "SimpleBlock".  If only one frame is present, "BlockLacing" must be None.  If more than one frame is present, "BlockLacing" must be one of: Xiph, Ebml, FixedSize.
/// 
#[derive(Clone, Debug)]
pub struct Frame {
    pub data: Vec<u8>
}

///
/// A typed interpretation of the Matroska "Block" element.
///
/// This struct has fields specific to the [Block](https://www.matroska.org/technical/basics.html#block-structure) element as defined by the [Matroska Spec](http://www.matroska.org/technical/specs/index.html).  This struct implements `TryFrom<MatroskaSpec>` and `Into<MatroskaSpec>` to simplify coercion to and from regular variants.
///
/// ## Example
///
/// ```
/// # use std::convert::TryInto;
/// use webm_iterable::matroska_spec::{MatroskaSpec, Block};
///
/// let variant = MatroskaSpec::Block(vec![0x83,0x00,0x01,0x9d,0x00,0x00,0x00]);
/// let mut block: Block = variant.try_into().unwrap();
/// assert_eq!(3, block.track);
/// ```
///
#[derive(Clone, Debug)]
pub struct Block {
    pub frames: Vec<Frame>,
    pub track: u64,
    pub timestamp: i16,

    pub invisible: bool,
    pub lacing: Option<BlockLacing>,
}

impl TryFrom<&[u8]> for Block {
    type Error = WebmCoercionError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let mut position: usize = 0;
        let (track, track_size) = ebml_tools::read_vint(data)
            .map_err(|_| WebmCoercionError::BlockCoercionError(String::from("Unable to read track data in Block.")))?
            .ok_or_else(|| WebmCoercionError::BlockCoercionError(String::from("Unable to read track data in Block.")))?;

        position += track_size;

        let value: [u8; 2] = data[position..position + 2].try_into()
            .map_err(|_| WebmCoercionError::BlockCoercionError(String::from("Attempting to create Block tag, but binary data length was not 2")))?;
        let timestamp = i16::from_be_bytes(value);
        position += 2;

        let flags: u8 = data[position];
        position += 1;
        let invisible = (flags & 0x10) == 0x10;

        let lacing: Option<BlockLacing>;
        if flags & 0x0c == 0x0c {
            lacing = Some(BlockLacing::FixedSize);
        } else if flags & 0x0c == 0x08 {
            lacing = Some(BlockLacing::Ebml);
        } else if flags & 0x0c == 0x04 {
            lacing = Some(BlockLacing::Xiph);
        } else {
            lacing = None;
        }

        let payload = &data[position..];

        Ok(Block {
            frames: get_block_frames(payload, lacing)?,
            track,
            timestamp,
            invisible,
            lacing,
        })
    }
}

fn get_block_frames(payload: &[u8], lacing: Option<BlockLacing>) -> Result<Vec<Frame>, WebmCoercionError> {
    if let Some(lacing) = lacing {
        let frame_count = payload[0] as usize + 1;
        let (mut frame_start, sizes) = match lacing {
            BlockLacing::Xiph => {
                let mut sizes: Vec<usize> = Vec::with_capacity(frame_count-1);
                let mut position: usize = 1;
                let mut size = 0;
                while sizes.len() < frame_count - 1 {
                    size += payload[position] as usize;
                    if payload[position] != 0xFF {
                        sizes.push(size);
                        size = 0;
                    }
                    position += 1;
                }
                Ok((position, sizes))
            },
            BlockLacing::Ebml => {
                let mut sizes: Vec<usize> = Vec::with_capacity(frame_count-1);
                let mut position: usize = 1;
                while sizes.len() < frame_count - 1 {
                    if let Some((val, val_len)) = ebml_tools::read_vint(&payload[position..]).ok().flatten() {
                        if let Some(last) = sizes.last() {
                            let difference = if val > ((1 << ((7 * val_len) - 1)) - 1) {
                                val | !((1 << (7 * val_len)) - 1)
                            } else {
                                val
                            } as i64;
                            sizes.push((difference + (*last as i64)) as usize);
                            position += val_len;
                        } else {
                            sizes.push(val as usize);
                            position += val_len;
                        }
                    } else {
                        return Err(WebmCoercionError::BlockCoercionError(String::from("Unable to read ebml lacing frame sizes in block")));
                    }
                }

                Ok((position, sizes))
            },
            BlockLacing::FixedSize => {
                let total_size = payload.len() - 1;
                if total_size % frame_count == 0 {
                    let frame_size = total_size / frame_count;
                    Ok((1usize, vec![frame_size; frame_count - 1]))
                } else {
                    Err(WebmCoercionError::BlockCoercionError(String::from("Block frame count with fixed lacing size did not match payload length")))
                }
            }
        }?;

        let mut frames: Vec<Frame> = Vec::with_capacity(frame_count);
        for size in sizes {
            frames.push(Frame { data: payload[frame_start..(frame_start+size)].to_vec() });
            frame_start += size;
        }
        frames.push(Frame { data: payload[frame_start..].to_vec() });

        Ok(frames)
    } else {
        Ok(vec![Frame { data: payload.to_vec() }])
    }
}

impl TryFrom<MatroskaSpec> for Block {
    type Error = WebmCoercionError;

    fn try_from(value: MatroskaSpec) -> Result<Self, Self::Error> {
        match value {
            MatroskaSpec::Block(data) => {
                let data: &[u8] = &data;
                Block::try_from(data)
            }
            _ => Err(WebmCoercionError::BlockCoercionError(String::from("Expected binary tag type for Block tag, but received a different type!"))),
        }
    }
}

impl From<Block> for MatroskaSpec {
    fn from(mut block: Block) -> Self {
        if block.frames.len() == 1 {
            // If there is only 1 frame, lacing doesn't apply
            block.lacing = None;
        } else if block.lacing.is_none() {
            // If there is more than 1 frame and lacing is not set, default to Ebml lacing
            block.lacing = Some(BlockLacing::Ebml);
        }

        let mut flags: u8 = 0x00;
        if block.invisible {
            flags |= 0x10;
        }

        if block.lacing.is_some() {
            match block.lacing.unwrap() {
                BlockLacing::Xiph => {
                    flags |= 0x04;
                }
                BlockLacing::Ebml => {
                    flags |= 0x08;
                }
                BlockLacing::FixedSize => {
                    flags |= 0x0c;
                }
            }
        }

        let payload = build_frame_payload(block.frames, block.lacing);

        let mut result = Vec::with_capacity(payload.len() + 11);
        result.extend_from_slice(&block.track.as_vint().expect("Unable to convert track value to vint"));
        result.extend_from_slice(&block.timestamp.to_be_bytes());
        result.extend_from_slice(&flags.to_be_bytes());
        result.extend_from_slice(&payload);

        MatroskaSpec::Block(result)
    }
}

pub fn build_frame_payload(frames: Vec<Frame>, lacing: Option<BlockLacing>) -> Vec<u8> {
    if let Some(lacing) = lacing {
        let sizes = match lacing {
            BlockLacing::Xiph => {
                let mut sizes: Vec<u8> = Vec::new();
                for frame in &frames[..frames.len()-1] {
                    sizes.resize(sizes.len() + frame.data.len()/255, 0xFF);
                    sizes.push((frame.data.len()%255) as u8);
                }
                sizes
            },
            BlockLacing::Ebml => {
                let mut last_size: Option<usize> = None;
                let mut sizes: Vec<u8> = Vec::new();
                for frame in &frames[..frames.len()-1] {
                    let size = frame.data.len();
                    let written_size = if let Some(last_size) = last_size {
                        let mut diff = (size as i64) - (last_size as i64);
                        if diff < 0 {
                            let mut length: usize = 1;
                            while length <= 8 {
                                if diff > -(1 << ((7 * length) - 1)) {
                                    break;
                                }
                                length += 1;
                            }
                            diff &= (1 << (7 * length)) - 1;
                        }
                        diff as u64
                    } else {
                        size as u64
                    };
                    sizes.append(&mut written_size.as_vint().unwrap());
                    last_size = Some(size);
                }
                sizes
            },
            BlockLacing::FixedSize => {
                vec![]
            }
        };

        let mut payload: Vec<u8> = Vec::with_capacity(1 + sizes.len() + frames.iter().fold(0, |a, c| a + c.data.len()));

        payload.push((frames.len()-1) as u8);
        payload.extend_from_slice(sizes.as_slice());
        for frame in frames {
            payload.extend_from_slice(frame.data.as_slice());
        }

        payload
    } else {
        frames[0].data.clone()
    }
}