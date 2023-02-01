use std::convert::{TryInto, TryFrom};

use ebml_iterable::tools::{self as ebml_tools, Vint};

use super::super::errors::WebmCoercionError;
use super::{Block, BlockLacing, Frame, MatroskaSpec};

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
    pub frames: Vec<Frame>,
    pub track: u64,
    /// The block timestamp
    pub timestamp: i16,

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
            frames: block.frames,
            track: block.track,
            timestamp: block.timestamp,
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
    fn from(mut simple_block: SimpleBlock) -> Self {
        if simple_block.frames.len() == 1 {
            // If there is only 1 frame, lacing doesn't apply
            simple_block.lacing = None;
        } else if simple_block.lacing.is_none() {
            // If there is more than 1 frame and lacing is not set, default to Ebml lacing
            simple_block.lacing = Some(BlockLacing::Ebml);
        }
        
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

        let payload = super::block::build_frame_payload(simple_block.frames, simple_block.lacing);

        let mut result = Vec::with_capacity(payload.len() + 11);
        result.extend_from_slice(&simple_block.track.as_vint().expect("Unable to convert track value to vint"));
        result.extend_from_slice(&simple_block.timestamp.to_be_bytes());
        result.extend_from_slice(&flags.to_be_bytes());
        result.extend_from_slice(&payload);

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
        let simple_block = SimpleBlock::try_from(MatroskaSpec::SimpleBlock(block_content.clone())).unwrap();

        assert!(simple_block.keyframe);
        assert!(simple_block.discardable);
        assert!(simple_block.invisible);
        assert_eq!(Some(BlockLacing::FixedSize), simple_block.lacing);
        assert_eq!(1, simple_block.track);
        assert_eq!(1, simple_block.timestamp);
        assert_eq!(2, simple_block.frames.len());

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
        let simple_block = SimpleBlock {
            frames: vec![Frame { data: vec![0x01, 0x02, 0x03] }],
            track: 1,
            timestamp: 15,
            invisible: false,
            discardable: false,
            keyframe: true,
            lacing: None
        };

        let encoded: MatroskaSpec = simple_block.clone().into();
        let redecoded = SimpleBlock::try_from(encoded).unwrap();

        assert_eq!(simple_block.keyframe, redecoded.keyframe);
        assert_eq!(simple_block.discardable, redecoded.discardable);
        assert_eq!(simple_block.invisible, redecoded.invisible);
        assert_eq!(simple_block.lacing, redecoded.lacing);
        assert_eq!(simple_block.track, redecoded.track);
        assert_eq!(simple_block.timestamp, redecoded.timestamp);
        for i in 0..simple_block.frames.len() {
            assert_eq!(simple_block.frames[i].data, redecoded.frames[i].data);
        }
    }

    #[test]
    fn encode_decode_simple_block_xiphlacing() {
        let simple_block = SimpleBlock {
            frames: vec![Frame { data: vec![0x01, 0x02, 0x03] }, Frame { data: vec![0x04, 0x05, 0x06] }, Frame { data: vec![0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e] }],
            track: 1,
            timestamp: 15,
            invisible: false,
            discardable: false,
            keyframe: true,
            lacing: Some(BlockLacing::Xiph)
        };

        let encoded: MatroskaSpec = simple_block.clone().into();
        let redecoded = SimpleBlock::try_from(encoded).unwrap();

        assert_eq!(simple_block.keyframe, redecoded.keyframe);
        assert_eq!(simple_block.discardable, redecoded.discardable);
        assert_eq!(simple_block.invisible, redecoded.invisible);
        assert_eq!(simple_block.lacing, redecoded.lacing);
        assert_eq!(simple_block.track, redecoded.track);
        assert_eq!(simple_block.timestamp, redecoded.timestamp);
        for i in 0..simple_block.frames.len() {
            assert_eq!(simple_block.frames[i].data, redecoded.frames[i].data);
        }
    }

    #[test]
    fn encode_decode_simple_block_ebmllacing() {
        let simple_block = SimpleBlock {
            frames: vec![Frame { data: vec![0x01, 0x02, 0x03] }, Frame { data: vec![0x04, 0x05, 0x06] }, Frame { data: vec![0x00] }, Frame { data: vec![0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e] }, Frame { data: vec![0x01, 0x02] }],
            track: 1,
            timestamp: 15,
            invisible: false,
            discardable: false,
            keyframe: true,
            lacing: Some(BlockLacing::Ebml)
        };

        let encoded: MatroskaSpec = simple_block.clone().into();
        let redecoded = SimpleBlock::try_from(encoded).unwrap();

        assert_eq!(simple_block.keyframe, redecoded.keyframe);
        assert_eq!(simple_block.discardable, redecoded.discardable);
        assert_eq!(simple_block.invisible, redecoded.invisible);
        assert_eq!(simple_block.lacing, redecoded.lacing);
        assert_eq!(simple_block.track, redecoded.track);
        assert_eq!(simple_block.timestamp, redecoded.timestamp);
        for i in 0..simple_block.frames.len() {
            assert_eq!(simple_block.frames[i].data, redecoded.frames[i].data);
        }
    }

    #[test]
    fn encode_decode_simple_block_fixedlacing() {
        let simple_block = SimpleBlock {
            frames: vec![Frame { data: vec![0x01, 0x02, 0x03] }, Frame { data: vec![0x04, 0x05, 0x06] }],
            track: 1,
            timestamp: 15,
            invisible: false,
            discardable: false,
            keyframe: true,
            lacing: Some(BlockLacing::FixedSize)
        };

        let encoded: MatroskaSpec = simple_block.clone().into();
        let redecoded = SimpleBlock::try_from(encoded).unwrap();

        assert_eq!(simple_block.keyframe, redecoded.keyframe);
        assert_eq!(simple_block.discardable, redecoded.discardable);
        assert_eq!(simple_block.invisible, redecoded.invisible);
        assert_eq!(simple_block.lacing, redecoded.lacing);
        assert_eq!(simple_block.track, redecoded.track);
        assert_eq!(simple_block.timestamp, redecoded.timestamp);
        for i in 0..simple_block.frames.len() {
            assert_eq!(simple_block.frames[i].data, redecoded.frames[i].data);
        }
    }
}