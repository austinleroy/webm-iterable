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
    pub payload: Vec<u8>,
    pub track: u64,
    pub value: i16,

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

        Ok(Block {
            payload,
            track,
            value,
            invisible,
            lacing,
        })
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
    fn from(block: Block) -> Self {
        let mut result = Vec::with_capacity(block.payload.len() + 11);
        result.extend_from_slice(&block.track.as_vint().expect("Unable to convert track value to vint"));
        result.extend_from_slice(&block.value.to_be_bytes());

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

        result.extend_from_slice(&flags.to_be_bytes());
        result.extend_from_slice(&block.payload);

        MatroskaSpec::Block(result)
    }
}
