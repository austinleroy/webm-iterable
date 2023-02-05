use std::convert::{TryFrom, TryInto};
use ebml_iterable::tools::{self as ebml_tools, Vint};

use crate::errors::WebmCoercionError;
use crate::MatroskaSpec;
use super::block_utils::{read_frame_data, write_frame_data};

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
pub struct Frame<'a> {
    pub data: &'a [u8]
}

///
/// A typed interpretation of the Matroska "Block" element.
///
/// This struct has fields specific to the [Block](https://www.matroska.org/technical/basics.html#block-structure) element as defined by the [Matroska Spec](http://www.matroska.org/technical/specs/index.html).  This struct implements `TryFrom<&MatroskaSpec>` and `Into<MatroskaSpec>` to simplify coercion to and from regular variants.
///
/// ## Example
///
/// ```
/// # use std::convert::TryInto;
/// use webm_iterable::matroska_spec::{MatroskaSpec, Block};
///
/// let variant = &MatroskaSpec::Block(vec![0x83,0x00,0x01,0x9d,0x00,0x00,0x00]);
/// let mut block: Block = variant.try_into().unwrap();
/// assert_eq!(3, block.track);
/// ```
///
#[derive(Clone, Debug)]
pub struct Block<'a> {
    /// Raw frame data used to create the block (avoids the extra allocation of using owned_frame_data)
    frame_data: &'a [u8],

    /// Owned frame data that can be set to allow changing frame data on the block
    owned_frame_data: Option<Vec<u8>>,

    pub track: u64,
    pub timestamp: i16,

    pub invisible: bool,
    pub lacing: Option<BlockLacing>,
}

impl<'a> Block<'a> {
    ///
    /// Reads the raw frame data of the block.
    /// 
    /// Frame data can be formatted differently depending on the block lacing.  Generally, it is easier to use [`Self::read_frame_data()`] rather than this method to access the frames in the block.  This method is provided in the event raw packet data needs to be handled in a special way (for example, if the data is encrypted).
    /// 
    pub fn raw_frame_data(&self) -> &[u8] {
        self.owned_frame_data.as_deref().unwrap_or(self.frame_data)
    }

    ///
    /// Reads the frames encoded in the block.
    /// 
    /// This method outputs the binary frames encoded in the block, taking into account any block lacing.  Details on block lacing can be found in the [Matroska spec](https://www.matroska.org/technical/notes.html).
    /// 
    /// # Errors
    /// 
    /// This method can return an error if the frame data is malformed.
    /// 
    pub fn read_frame_data(&self) -> Result<Vec<Frame>, WebmCoercionError> {
        read_frame_data(self.owned_frame_data.as_deref().unwrap_or(self.frame_data), &self.lacing)
    }

    ///
    /// Updates the frame data contained in the block.
    /// 
    /// This method writes frame data to a newly allocated vector owned by the block.  Future calls to [`Self::read_frame_data()`] and [`Self::raw_frame_data()`] will use the data set via this method.
    /// 
    /// # Panics
    /// 
    /// This method can panic if the block has its lacing set as ['BlockLacing::FixedSize`] and the input frames are not all the same length.
    /// 
    pub fn set_frame_data(&mut self, frames: &Vec<Frame>) {
        let (data, new_lacing) = write_frame_data(frames, self.lacing);
        self.lacing = new_lacing;
        self.owned_frame_data = Some(data);
    }
}

impl<'a> TryFrom<&'a Vec<u8>> for Block<'a> {
    type Error = WebmCoercionError;

    fn try_from(value: &'a Vec<u8>) -> Result<Self, Self::Error> {
       value.as_slice().try_into()
    }
}

impl<'a> TryFrom<&'a [u8]> for Block<'a> {
    type Error = WebmCoercionError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
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
        let invisible = (flags & 0x08) == 0x08;

        let lacing: Option<BlockLacing>;
        if flags & 0x06 == 0x06 {
            lacing = Some(BlockLacing::Ebml);
        } else if flags & 0x06 == 0x04 {
            lacing = Some(BlockLacing::FixedSize);
        } else if flags & 0x06 == 0x02 {
            lacing = Some(BlockLacing::Xiph);
        } else {
            lacing = None;
        }

        let payload = &data[position..];

        Ok(Block {
            frame_data: payload,
            owned_frame_data: None,
            track,
            timestamp,
            invisible,
            lacing,
        })
    }
}

impl<'a> TryFrom<&'a MatroskaSpec> for Block<'a> {
    type Error = WebmCoercionError;

    fn try_from(value: &'a MatroskaSpec) -> Result<Self, Self::Error> {
        match value {
            MatroskaSpec::Block(data) => {
                Block::try_from(data.as_slice())
            }
            _ => Err(WebmCoercionError::BlockCoercionError(String::from("Expected binary tag type for Block tag, but received a different type!"))),
        }
    }
}

impl<'a> From<Block<'a>> for MatroskaSpec {
    fn from(block: Block) -> Self {
        let mut flags: u8 = 0x00;
        if block.invisible {
            flags |= 0x08;
        }

        if block.lacing.is_some() {
            match block.lacing.unwrap() {
                BlockLacing::Xiph => {
                    flags |= 0x02;
                }
                BlockLacing::Ebml => {
                    flags |= 0x06;
                }
                BlockLacing::FixedSize => {
                    flags |= 0x04;
                }
            }
        }

        let data = block.owned_frame_data.as_deref().unwrap_or(block.frame_data);
        let mut result = Vec::with_capacity(data.len() + 11);
        result.extend_from_slice(&block.track.as_vint().expect("Unable to convert track value to vint"));
        result.extend_from_slice(&block.timestamp.to_be_bytes());
        result.extend_from_slice(&flags.to_be_bytes());
        result.extend_from_slice(data);

        MatroskaSpec::Block(result)
    }
}