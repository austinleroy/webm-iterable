use std::convert::{TryInto, TryFrom};

use ebml_iterable::tools::{self as ebml_tools, Vint};

use super::super::errors::WebmError;
use super::{Block, BlockLacing, MatroskaSpec};

///
/// A typed interpretation of the Matroska "SimpleBlock" element.
/// 
/// This struct has fields specific to the [SimpleBlock](https://www.matroska.org/technical/basics.html#simpleblock-structure) element as defined by the [Matroska Spec](http://www.matroska.org/technical/specs/index.html).  This struct implements `TryFrom<TagData>` and `Into<TagData>` to simplify coercion to and from regular [`TagData::Binary`] values.
/// 
/// ## Example
/// 
/// ```
/// # use std::convert::TryInto;
/// # use ebml_iterable::tags::TagData;
/// use webm_iterable::matroska_spec::SimpleBlock;
/// 
/// let binary_tag_data = TagData::Binary(vec![0x81,0x00,0x01,0x9d,0x00,0x00,0x00]);
/// let mut simple_block: SimpleBlock = binary_tag_data.try_into().unwrap();
/// simple_block.discardable = true;
/// ```
/// 
pub struct SimpleBlock {
    pub block: Block,
    pub discardable: bool,
    pub keyframe: bool,
}

impl TryFrom<MatroskaSpec> for SimpleBlock {
    type Error = WebmError;

    fn try_from(value: MatroskaSpec) -> Result<Self, Self::Error> {
        if let MatroskaSpec::SimpleBlock(data) = &value {
            let data = &data;
            let mut position: usize = 0;
            let (track, track_size) = ebml_tools::read_vint(data)
                .map_err(|_| WebmError::BlockCoercionError(String::from("Unable to read track data in Block.")))?
                .ok_or_else(|| WebmError::BlockCoercionError(String::from("Unable to read track data in Block.")))?;

            position += track_size;

            let value: [u8;2] = data[position..position+2].try_into()
                .map_err(|_| WebmError::BlockCoercionError(String::from("Attempting to create Block tag, but binary data length was not 2")))?;
            let value = i16::from_be_bytes(value);
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
            
            let payload = data[position..].to_vec();
            let mut position: usize = 0;
            let (_track, track_size) = ebml_tools::read_vint(data)
                .map_err(|_| WebmError::SimpleBlockCoercionError(String::from("Unable to read track data in SimpleBlock.")))?
                .ok_or_else(|| WebmError::SimpleBlockCoercionError(String::from("Unable to read track data in SimpleBlock.")))?;

            position += track_size + 2;
            let flags: u8 = data[position];

            let keyframe = flags & 0x80 == 0x80;
            let discardable = flags & 0x01 == 0x01;

            Ok(SimpleBlock {
                block: Block {
                    payload,
                    track,
                    value,
                    invisible,
                    lacing,
                },
                discardable,
                keyframe,
            })
        } else {
            Err(WebmError::SimpleBlockCoercionError(String::from("Expected binary tag type for SimpleBlock tag, but received a different type!")))
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<MatroskaSpec> for SimpleBlock {
    fn into(self) -> MatroskaSpec {
        let mut result = Vec::with_capacity(self.block.payload.len() + 11);
        result.extend_from_slice(&self.block.track.as_vint().expect("Unable to convert track value to vint"));
        result.extend_from_slice(&self.block.value.to_be_bytes());
        
        let mut flags: u8 = 0x00;
        if self.block.invisible {
          flags |= 0x10;
        }
        
        if self.block.lacing.is_some() {
          match self.block.lacing.unwrap() {
            BlockLacing::Xiph => { flags |= 0x04; },
            BlockLacing::Ebml => { flags |= 0x08; },
            BlockLacing::FixedSize => { flags |= 0x0c; },
          }
        }

        if self.discardable {
            flags |= 0x01;
        }

        if self.keyframe {
            flags |= 0x80;
        }

        result.extend_from_slice(&flags.to_be_bytes());
        result.extend_from_slice(&self.block.payload);

        MatroskaSpec::SimpleBlock(result)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::SimpleBlock;
    use super::TagData;
    use super::super::block::BlockLacing;

    #[test]
    fn decode_encode_simple_block() {
        let block_content = vec![0x81,0x00,0x01,0x9d,0x00,0x00,0x00];
        let simple_block = SimpleBlock::try_from(TagData::Binary(block_content.clone())).unwrap();

        assert!(simple_block.keyframe);
        assert!(simple_block.discardable);
        assert!(simple_block.block.invisible);
        assert_eq!(Some(BlockLacing::FixedSize), simple_block.block.lacing);
        assert_eq!(1, simple_block.block.track);
        assert_eq!(1, simple_block.block.value);

        let encoded: TagData = simple_block.into();

        match encoded {
            TagData::Binary(data) => {
                assert_eq!(block_content, data);
            },
            _ => panic!("not binary type?"),
        }
    }
}