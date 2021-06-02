use matroska_spec::MatroskaSpec;
use ebml_iterable::{TagIterator, TagWriter};

pub mod tags {
    pub use ebml_iterable::tags::{DataTag, DataTagType, EbmlTag};
    pub use ebml_iterable::specs::TagSpec as TagSpec;
}

mod errors;
pub mod matroska_spec;
pub type WebmIterator<'a> = TagIterator<'a, MatroskaSpec>;
pub type WebmWriter<'a> = TagWriter<'a>;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::tags::{EbmlTag, DataTag, DataTagType};
    use super::WebmWriter;
    use super::WebmIterator;

    #[test]
    fn basic_tag_stream_write_and_iterate() {
        let tags: Vec<EbmlTag> = vec![
            EbmlTag::StartTag(0x1a45dfa3),
            EbmlTag::StartTag(0x18538067),
            EbmlTag::FullTag(DataTag { id: 0x83, data_type: DataTagType::UnsignedInt(0x01) }),
            EbmlTag::EndTag(0x18538067),
            EbmlTag::FullTag(DataTag { id: 0x1f43b675, data_type: DataTagType::Master(vec![
                DataTag { id: 0x97, data_type: DataTagType::UnsignedInt(0x02) }
            ])}),
            EbmlTag::EndTag(0x1a45dfa3),
        ];

        let mut dest = Cursor::new(Vec::new());
        let mut writer = WebmWriter::new(&mut dest);

        for tag in tags {
            writer.write(tag).expect("Test shouldn't error");
        }

        println!("dest {:?}", dest);

        let mut src = Cursor::new(dest.get_ref().to_vec());
        let reader = WebmIterator::new(&mut src, &[]);
        let tags: Vec<EbmlTag> = reader.map(|i| i.unwrap().tag).collect();

        println!("tags {:?}", tags);

        assert_eq!(EbmlTag::StartTag(0x1a45dfa3), tags[0]);
        assert_eq!(EbmlTag::StartTag(0x18538067), tags[1]);
        assert_eq!(EbmlTag::FullTag(DataTag { id: 0x83, data_type: DataTagType::UnsignedInt(0x01) }), tags[2]);
        assert_eq!(EbmlTag::EndTag(0x18538067), tags[3]);
        assert_eq!(EbmlTag::StartTag(0x1f43b675), tags[4]);
        assert_eq!(EbmlTag::FullTag(DataTag { id: 0x97, data_type: DataTagType::UnsignedInt(0x02) }), tags[5]);
        assert_eq!(EbmlTag::EndTag(0x1f43b675), tags[6]);
        assert_eq!(EbmlTag::EndTag(0x1a45dfa3), tags[7]);
    }
}