use std::convert::{TryInto, TryFrom};

use ebml_iterable::tools as ebml_tools;
use ebml_iterable::tags::DataTagType;

use super::super::errors::WebmError;
use super::Block;

pub struct SimpleBlock {
    pub block: Block,
    pub discardable: bool,
    pub keyframe: bool,
}

impl TryFrom<DataTagType> for SimpleBlock {
    type Error = WebmError;

    fn try_from(value: DataTagType) -> Result<Self, Self::Error> {
        if let DataTagType::Binary(data) = &value {
            let data = &data;
            let mut position: usize = 0;
            let (_track, track_size) = ebml_tools::read_vint(data)
                .map_err(|_| WebmError::SimpleBlockCoercionError(String::from("Unable to read track data in SimpleBlock.")))?
                .ok_or_else(|| WebmError::SimpleBlockCoercionError(String::from("Unable to read track data in SimpleBlock.")))?;

            position += track_size + 2;
            let flags: u8 = data[position];

            let keyframe = flags & 0x80 == 0x80;
            let discardable = flags & 0x01 == 0x01;

            Ok(SimpleBlock {
                block: value.try_into()?,
                discardable,
                keyframe,
            })
        } else {
            Err(WebmError::SimpleBlockCoercionError(String::from("Expected binary tag type for SimpleBlock tag, but received a different type!")))
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<DataTagType> for SimpleBlock {
    fn into(self) -> DataTagType {
        let mut result: DataTagType = self.block.into();

        match result {
            DataTagType::Binary(ref mut data) => {
                let mut position: usize = 0;
                let (_track, track_size) = ebml_tools::read_vint(&data)
                    .expect("Invalid data passed to block.  Could not read track.")
                    .expect("Invalid data passed to block.  Could not read track.");
                
                position += track_size + 2;
                let flags = &mut data[position];

                if self.discardable {
                    *flags |= 0x01;
                }

                if self.keyframe {
                    *flags |= 0x80;
                }
            },
            _ => panic!("Never should reach here - block was not binary tag type"),
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::SimpleBlock;
    use super::DataTagType;
    use super::super::block::BlockLacing;

    #[test]
    fn decode_encode_simple_block() {
        let block_content = vec![0x81,0x00,0x01,0x9d,0x00,0x00,0x00];
        let simple_block = SimpleBlock::try_from(DataTagType::Binary(block_content.clone())).unwrap();

        assert!(simple_block.keyframe);
        assert!(simple_block.discardable);
        assert!(simple_block.block.invisible);
        assert_eq!(BlockLacing::FixedSize, simple_block.block.lacing);
        assert_eq!(1, simple_block.block.track);
        assert_eq!(1, simple_block.block.value);

        let encoded: DataTagType = simple_block.into();

        match encoded {
            DataTagType::Binary(data) => {
                assert_eq!(block_content, data);
            },
            _ => panic!("not binary type?"),
        }
    }
}