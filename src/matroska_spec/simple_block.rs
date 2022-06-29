use std::convert::{TryInto, TryFrom};

use ebml_iterable::tools::{self as ebml_tools, Vint};

use super::super::errors::WebmCoercionError;
use super::{Block, BlockLacing, MatroskaSpec};

///
/// A typed interpretation of the Matroska "SimpleBlock" element.
/// 
/// This struct has fields specific to the [SimpleBlock](https://www.matroska.org/technical/basics.html#simpleblock-structure) element as defined by the [Matroska Spec](http://www.matroska.org/technical/specs/index.html).  This struct implements `TryFrom<MatroskaSpec>` and `Into<MatroskaSpec>` to simplify coercion to and from regular enum variants.
/// 
/// ## Example
/// 
/// ```
/// # use std::convert::TryInto;
/// use webm_iterable::matroska_spec::{MatroskaSpec, SimpleBlock};
/// 
/// let variant = MatroskaSpec::SimpleBlock(vec![0x81,0x00,0x01,0x9d,0x00,0x00,0x00]);
/// let mut simple_block: SimpleBlock = variant.try_into().unwrap();
/// assert_eq!(true, simple_block.discardable);
/// ```
/// 
#[derive(Clone, Debug)]
pub struct SimpleBlock {
    pub payload: Vec<u8>,
    pub track: u64,
    /// The block timestamp
    pub value: i16,

    pub invisible: bool,
    pub lacing: Option<BlockLacing>,
    pub discardable: bool,
    pub keyframe: bool,
}

impl TryFrom<&Vec<u8>> for SimpleBlock {
    type Error = WebmCoercionError;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
       value.as_slice().try_into()
    }
}

impl TryFrom<&[u8]> for SimpleBlock {
    type Error = WebmCoercionError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let block: Block = data.try_into()?;
        let mut position: usize = 0;
        let (_track, track_size) = ebml_tools::read_vint(data)
            .map_err(|_| WebmCoercionError::SimpleBlockCoercionError(String::from("Unable to read track data in SimpleBlock.")))?
            .ok_or_else(|| WebmCoercionError::SimpleBlockCoercionError(String::from("Unable to read track data in SimpleBlock.")))?;

        position += track_size + 2;
        let flags: u8 = data[position];

        let keyframe = flags & 0x80 == 0x80;
        let discardable = flags & 0x01 == 0x01;

        Ok(SimpleBlock {
            payload: block.payload,
            track: block.track,
            value: block.value,
            invisible: block.invisible,
            lacing: block.lacing,
            discardable,
            keyframe,
        })
    }
}

impl TryFrom<MatroskaSpec> for SimpleBlock {
    type Error = WebmCoercionError;

    fn try_from(value: MatroskaSpec) -> Result<Self, Self::Error> {
        match value {
            MatroskaSpec::SimpleBlock(data) => {
                let data: &[u8] = &data;
                SimpleBlock::try_from(data)
            },
            _ => Err(WebmCoercionError::SimpleBlockCoercionError(String::from("Only 'SimpleBlock' variants can be converted to a SimpleBlock struct")))
        }
    }
}

impl From<SimpleBlock> for MatroskaSpec {
    fn from(simple_block: SimpleBlock) -> Self {
        let mut result = Vec::with_capacity(simple_block.payload.len() + 11);
        result.extend_from_slice(&simple_block.track.as_vint().expect("Unable to convert track value to vint"));
        result.extend_from_slice(&simple_block.value.to_be_bytes());
        
        let mut flags: u8 = 0x00;
        if simple_block.invisible {
          flags |= 0x10;
        }
        
        if simple_block.lacing.is_some() {
          match simple_block.lacing.unwrap() {
            BlockLacing::Xiph => { flags |= 0x04; },
            BlockLacing::Ebml => { flags |= 0x08; },
            BlockLacing::FixedSize => { flags |= 0x0c; },
          }
        }

        if simple_block.discardable {
            flags |= 0x01;
        }

        if simple_block.keyframe {
            flags |= 0x80;
        }

        result.extend_from_slice(&flags.to_be_bytes());
        result.extend_from_slice(&simple_block.payload);

        MatroskaSpec::SimpleBlock(result)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::MatroskaSpec;
    use super::SimpleBlock;
    use super::super::block::BlockLacing;

    #[test]
    fn decode_encode_simple_block() {
        let block_content = vec![0x81,0x00,0x01,0x9d,0x00,0x00,0x00];
        let simple_block = SimpleBlock::try_from(MatroskaSpec::SimpleBlock(block_content.clone())).unwrap();

        assert!(simple_block.keyframe);
        assert!(simple_block.discardable);
        assert!(simple_block.invisible);
        assert_eq!(Some(BlockLacing::FixedSize), simple_block.lacing);
        assert_eq!(1, simple_block.track);
        assert_eq!(1, simple_block.value);

        let encoded: MatroskaSpec = simple_block.into();

        match encoded {
            MatroskaSpec::SimpleBlock(data) => {
                assert_eq!(block_content, data);
            },
            _ => panic!("not simple block variant?"),
        }
    }
}