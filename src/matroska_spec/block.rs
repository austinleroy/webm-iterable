use std::convert::{TryInto, TryFrom};

use ebml_iterable::tools::{self as ebml_tools, Vint};
use ebml_iterable::tags::TagData;

use super::super::errors::WebmError;

///
/// An enum describing different block lacing options.
/// 
/// This enum is based on the definition for [Lacing](https://www.matroska.org/technical/basics.html#lacing) as defined by the [Matroska Spec](http://www.matroska.org/technical/specs/index.html).
/// 
#[derive(PartialEq, Debug)]
pub enum BlockLacing {
    Xiph,
    Ebml,
    FixedSize,
}

///
/// A typed interpretation of the Matroska "Block" element.
/// 
/// This struct has fields specific to the [Block](https://www.matroska.org/technical/basics.html#block-structure) element as defined by the [Matroska Spec](http://www.matroska.org/technical/specs/index.html).  This struct implements `TryFrom<TagData>` and `Into<TagData>` to simplify coercion to and from regular [`TagData::Binary`] values.
/// 
/// ## Example
/// 
/// ```
/// # use std::convert::TryInto;
/// # use ebml_iterable::tags::TagData;
/// use webm_iterable::matroska_spec::Block;
/// 
/// let binary_tag_data = TagData::Binary(vec![0x81,0x00,0x01,0x9d,0x00,0x00,0x00]);
/// let mut block: Block = binary_tag_data.try_into().unwrap();
/// block.track = 3;
/// ```
/// 
pub struct Block {
    pub payload: Vec<u8>,
    pub track: u64,
    pub value: i16,

    pub invisible: bool,
    pub lacing: Option<BlockLacing>,
}

impl TryFrom<TagData> for Block {
  type Error = WebmError;

  fn try_from(value: TagData) -> Result<Self, Self::Error> {
      if let TagData::Binary(data) = value {
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

          Ok(Block {
            payload,
            track,
            value,
            invisible,
            lacing
          })
      } else {
          Err(WebmError::BlockCoercionError(String::from("Expected binary tag type for Block tag, but received a different type!")))
      }
  }
}

#[allow(clippy::from_over_into)]
impl Into<TagData> for Block {
    fn into(self) -> TagData {
        let mut result = Vec::with_capacity(self.payload.len() + 11);
        result.extend_from_slice(&self.track.as_vint().expect("Unable to convert track value to vint"));
        result.extend_from_slice(&self.value.to_be_bytes());
        
        let mut flags: u8 = 0x00;
        if self.invisible {
          flags |= 0x10;
        }
        
        if self.lacing.is_some() {
          match self.lacing.unwrap() {
            BlockLacing::Xiph => { flags |= 0x04; },
            BlockLacing::Ebml => { flags |= 0x08; },
            BlockLacing::FixedSize => { flags |= 0x0c; },
          }
        }

        result.extend_from_slice(&flags.to_be_bytes());
        result.extend_from_slice(&self.payload);
        
        TagData::Binary(result)
    }
}