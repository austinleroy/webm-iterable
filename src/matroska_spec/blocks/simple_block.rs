use std::convert::{TryInto, TryFrom};

use ebml_iterable::tools::{self as ebml_tools, Vint};

use crate::{MatroskaSpec, errors::WebmCoercionError};
use super::block::{Block, BlockLacing, Frame};
use super::block_utils::{read_frame_data, write_frame_data};

///
/// A typed interpretation of the Matroska "SimpleBlock" element.
/// 
/// This struct has fields specific to the [SimpleBlock](https://www.matroska.org/technical/basics.html#simpleblock-structure) element as defined by the [Matroska Spec](http://www.matroska.org/technical/specs/index.html).  This struct implements `TryFrom<&MatroskaSpec>` and `Into<MatroskaSpec>` to simplify coercion to and from regular enum variants.
/// 
/// ## Example
/// 
/// ```
/// # use std::convert::TryInto;
/// use webm_iterable::matroska_spec::{MatroskaSpec, SimpleBlock};
/// 
/// let variant = &MatroskaSpec::SimpleBlock(vec![0x81,0x00,0x01,0x9d,0x00,0x00,0x00]);
/// let mut simple_block: SimpleBlock = variant.try_into().unwrap();
/// assert_eq!(true, simple_block.discardable);
/// ```
/// 
#[derive(Clone, Debug)]
pub struct SimpleBlock<'a> {
    /// Raw frame data used to create the simple block (avoids the extra allocation of using owned_frame_data)
    frame_data: &'a [u8],

    /// Owned frame data that can be set to allow changing frame data on the simple block
    owned_frame_data: Option<Vec<u8>>,

    pub track: u64,
    pub timestamp: i16,

    pub invisible: bool,
    pub lacing: Option<BlockLacing>,
    pub discardable: bool,
    pub keyframe: bool,
}

impl<'a> SimpleBlock<'a> {
    ///
    /// Reads the raw frame data of the simple block.
    /// 
    /// Frame data can be formatted differently depending on the block lacing.  Generally, it is easier to use [`Self::read_frame_data()`] rather than this method to access the frames in the block.  This method is provided in the event raw packet data needs to be handled in a special way (for example, if the data is encrypted).
    /// 
    pub fn raw_frame_data(&self) -> &[u8] {
        self.owned_frame_data.as_deref().unwrap_or(self.frame_data)
    }

    ///
    /// Reads the frames encoded in the simple block.
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
    /// Updates the frame data contained in the simple block.
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

    ///
    /// Creates a new simple block with the given data.
    /// 
    /// Primarily used when you want to write with a given frame.
    /// For example, when you want to remux a video with libvpx.
    /// 
    /// # Safety
    /// The frame data is not checked for validity.
    /// 
    pub fn new_uncheked(frame_data: &'a [u8], track: u64, timestamp: i16, invisible: bool, lacing: Option<BlockLacing>, discardable: bool, keyframe: bool) -> Self {
        SimpleBlock {
            frame_data,
            owned_frame_data: None,
            track,
            timestamp,
            invisible,
            lacing,
            discardable,
            keyframe,
        }
    }
}

impl<'a> TryFrom<&'a Vec<u8>> for SimpleBlock<'a> {
    type Error = WebmCoercionError;

    fn try_from(value: &'a Vec<u8>) -> Result<Self, Self::Error> {
       value.as_slice().try_into()
    }
}

impl<'a> TryFrom<&'a [u8]> for SimpleBlock<'a> {
    type Error = WebmCoercionError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let block: Block = data.try_into()?;
        let mut position: usize = 0;
        let (_track, track_size) = ebml_tools::read_vint(data)
            .map_err(|_| WebmCoercionError::SimpleBlockCoercionError(String::from("Unable to read track data in SimpleBlock.")))?
            .ok_or_else(|| WebmCoercionError::SimpleBlockCoercionError(String::from("Unable to read track data in SimpleBlock.")))?;

        position += track_size + 2;
        let flags: u8 = data[position];
        position += 1;

        let keyframe = flags & 0x80 == 0x80;
        let discardable = flags & 0x01 == 0x01;

        Ok(SimpleBlock {
            frame_data: &data[position..],
            owned_frame_data: None,
            track: block.track,
            timestamp: block.timestamp,
            invisible: block.invisible,
            lacing: block.lacing,
            discardable,
            keyframe,
        })
    }
}

impl<'a> TryFrom<&'a MatroskaSpec> for SimpleBlock<'a> {
    type Error = WebmCoercionError;

    fn try_from(value: &'a MatroskaSpec) -> Result<Self, Self::Error> {
        match value {
            MatroskaSpec::SimpleBlock(data) => {
                SimpleBlock::try_from(data.as_slice())
            },
            _ => Err(WebmCoercionError::SimpleBlockCoercionError(String::from("Only 'SimpleBlock' variants can be converted to a SimpleBlock struct")))
        }
    }
}

impl<'a> From<SimpleBlock<'a>> for MatroskaSpec {
    fn from(simple_block: SimpleBlock) -> Self {        
        let mut flags: u8 = 0x00;
        if simple_block.invisible {
          flags |= 0x08;
        }
        
        if simple_block.lacing.is_some() {
          match simple_block.lacing.unwrap() {
            BlockLacing::Xiph => { flags |= 0x02; },
            BlockLacing::Ebml => { flags |= 0x06; },
            BlockLacing::FixedSize => { flags |= 0x04; },
          }
        }

        if simple_block.discardable {
            flags |= 0x01;
        }

        if simple_block.keyframe {
            flags |= 0x80;
        }

        let data = simple_block.owned_frame_data.as_deref().unwrap_or(simple_block.frame_data);
        let mut result = Vec::with_capacity(data.len() + 11);
        result.extend_from_slice(&simple_block.track.as_vint().expect("Unable to convert track value to vint"));
        result.extend_from_slice(&simple_block.timestamp.to_be_bytes());
        result.extend_from_slice(&flags.to_be_bytes());
        result.extend_from_slice(data);

        MatroskaSpec::SimpleBlock(result)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::MatroskaSpec;
    use super::SimpleBlock;
    use super::Frame;
    use super::BlockLacing;

    #[test]
    fn decode_encode_simple_block() {
        let block_content = vec![0x81,0x00,0x01,0x8d,0x01,0x00,0x00];
        let copy = MatroskaSpec::SimpleBlock(block_content.clone());
        let simple_block = SimpleBlock::try_from(&copy).unwrap();

        assert!(simple_block.keyframe);
        assert!(simple_block.discardable);
        assert!(simple_block.invisible);
        assert_eq!(Some(BlockLacing::FixedSize), simple_block.lacing);
        assert_eq!(1, simple_block.track);
        assert_eq!(1, simple_block.timestamp);
        assert_eq!(2, simple_block.read_frame_data().unwrap().len());

        let encoded: MatroskaSpec = simple_block.into();

        match encoded {
            MatroskaSpec::SimpleBlock(data) => {
                assert_eq!(block_content, data);
            },
            _ => panic!("not simple block variant?"),
        }
    }

    #[test]
    fn encode_decode_simple_block_nolacing() {
        let frames = vec![Frame { data: &[0x01, 0x02, 0x03] }];
        let mut simple_block = SimpleBlock {
            frame_data: &[],
            owned_frame_data: None,
            track: 1,
            timestamp: 15,
            invisible: false,
            discardable: false,
            keyframe: true,
            lacing: None
        };
        simple_block.set_frame_data(&frames);

        let encoded: MatroskaSpec = simple_block.clone().into();
        let redecoded = SimpleBlock::try_from(&encoded).unwrap();

        assert_eq!(simple_block.keyframe, redecoded.keyframe);
        assert_eq!(simple_block.discardable, redecoded.discardable);
        assert_eq!(simple_block.invisible, redecoded.invisible);
        assert_eq!(simple_block.lacing, redecoded.lacing);
        assert_eq!(simple_block.track, redecoded.track);
        assert_eq!(simple_block.timestamp, redecoded.timestamp);
        let redecoded_data = redecoded.read_frame_data().unwrap();
        for i in 0..frames.len() {
            assert_eq!(frames[i].data, redecoded_data[i].data);
        }
    }

    #[test]
    fn encode_decode_simple_block_xiphlacing() {
        let frames = vec![Frame { data: &[0x01, 0x02, 0x03] }, Frame { data: &[0x04, 0x05, 0x06] }, Frame { data: &[0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e] }];
        let mut simple_block = SimpleBlock {
            frame_data: &[],
            owned_frame_data: None,
            track: 1,
            timestamp: 15,
            invisible: false,
            discardable: false,
            keyframe: true,
            lacing: Some(BlockLacing::Xiph)
        };
        simple_block.set_frame_data(&frames);

        let encoded: MatroskaSpec = simple_block.clone().into();
        let redecoded = SimpleBlock::try_from(&encoded).unwrap();

        assert_eq!(simple_block.keyframe, redecoded.keyframe);
        assert_eq!(simple_block.discardable, redecoded.discardable);
        assert_eq!(simple_block.invisible, redecoded.invisible);
        assert_eq!(simple_block.lacing, redecoded.lacing);
        assert_eq!(simple_block.track, redecoded.track);
        assert_eq!(simple_block.timestamp, redecoded.timestamp);
        let redecoded_data = redecoded.read_frame_data().unwrap();
        for i in 0..frames.len() {
            assert_eq!(frames[i].data, redecoded_data[i].data);
        }
    }

    #[test]
    fn encode_decode_simple_block_ebmllacing() {
        let frames = vec![Frame { data: &[0x01, 0x02, 0x03] }, Frame { data: &[0x04, 0x05, 0x06] }, Frame { data: &[0x00] }, Frame { data: &[0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e] }, Frame { data: &[0x01, 0x02] }];
        let mut simple_block = SimpleBlock {
            frame_data: &[],
            owned_frame_data: None,
            track: 1,
            timestamp: 15,
            invisible: false,
            discardable: false,
            keyframe: true,
            lacing: Some(BlockLacing::Ebml)
        };
        simple_block.set_frame_data(&frames);

        let encoded: MatroskaSpec = simple_block.clone().into();
        let redecoded = SimpleBlock::try_from(&encoded).unwrap();

        assert_eq!(simple_block.keyframe, redecoded.keyframe);
        assert_eq!(simple_block.discardable, redecoded.discardable);
        assert_eq!(simple_block.invisible, redecoded.invisible);
        assert_eq!(simple_block.lacing, redecoded.lacing);
        assert_eq!(simple_block.track, redecoded.track);
        assert_eq!(simple_block.timestamp, redecoded.timestamp);
        let redecoded_data = redecoded.read_frame_data().unwrap();
        for i in 0..frames.len() {
            assert_eq!(frames[i].data, redecoded_data[i].data);
        }
    }

    #[test]
    fn encode_decode_simple_block_fixedlacing() {
        let frames = vec![Frame { data: &[0x01, 0x02, 0x03] }, Frame { data: &[0x04, 0x05, 0x06] }];
        let mut simple_block = SimpleBlock {
            frame_data: &[],
            owned_frame_data: None,
            track: 1,
            timestamp: 15,
            invisible: false,
            discardable: false,
            keyframe: true,
            lacing: Some(BlockLacing::FixedSize)
        };
        simple_block.set_frame_data(&frames);

        let encoded: MatroskaSpec = simple_block.clone().into();
        let redecoded = SimpleBlock::try_from(&encoded).unwrap();

        assert_eq!(simple_block.keyframe, redecoded.keyframe);
        assert_eq!(simple_block.discardable, redecoded.discardable);
        assert_eq!(simple_block.invisible, redecoded.invisible);
        assert_eq!(simple_block.lacing, redecoded.lacing);
        assert_eq!(simple_block.track, redecoded.track);
        assert_eq!(simple_block.timestamp, redecoded.timestamp);
        let redecoded_data = redecoded.read_frame_data().unwrap();
        for i in 0..frames.len() {
            assert_eq!(frames[i].data, redecoded_data[i].data);
        }
    }
}