//!
//! This crate was built to ease parsing files encoded in a Matroska container, such as [WebMs][webm] or [MKVs][mkv].
//!
//! The main content provided by this crate is the [`MatroskaSpec`] enum.  Otherwise, this crate simply provides type aliases in the form of [`WebmIterator`] and [`WebmWriter`].
//! 
//! [webm]: https://www.webmproject.org/
//! [mkv]: http://www.matroska.org/technical/specs/index.html
//! 
//!
//! # Example Usage
//! 
//! The following examples show how to read, modify, and write webm files using this library.
//! 
//! ## Example 1
//! The following example reads a media file into memory and decodes it.
//! 
//! ```
//! use std::fs::File;
//! use webm_iterable::WebmIterator;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut src = File::open("media/test.webm").unwrap();
//!     let tag_iterator = WebmIterator::new(&mut src, &[]);
//! 
//!     for tag in tag_iterator {
//!         println!("[{:?}]", tag?.spec_tag);
//!     }
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ## Example 2
//! This next example does the same thing, but keeps track of the number of times each tag appears in the file.
//! 
//! ```
//! use std::fs::File;
//! use std::collections::HashMap;
//! use webm_iterable::WebmIterator;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut src = File::open("media/test.webm").unwrap();
//!     let tag_iterator = WebmIterator::new(&mut src, &[]);
//!     let mut tag_counts = HashMap::new();
//! 
//!     for tag in tag_iterator {
//!         let count = tag_counts.entry(tag?.spec_tag).or_insert(0);
//!         *count += 1;
//!     }
//!     
//!     println!("{:?}", tag_counts);
//!     Ok(())
//! }
//! ```
//! 
//! ## Example 3
//! This example grabs the audio from a webm and stores the result in a new file.  The logic in this example is rather advanced - an explanation follows the code.
//! 
//! ```no_run
//! use std::fs::File;
//! use std::convert::TryInto;
//! 
//! use webm_iterable::{
//!     WebmIterator, 
//!     WebmWriter,
//!     matroska_spec::{MatroskaSpec, Block, EbmlSpecification},
//!     tags::{TagPosition, TagData},
//! };
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 1
//!     let mut src = File::open("media/audiosample.webm").unwrap();
//!     let tag_iterator = WebmIterator::new(&mut src, &[MatroskaSpec::TrackEntry]);
//!     let mut dest = File::create("media/audioout.webm").unwrap();
//!     let mut tag_writer = WebmWriter::new(&mut dest);
//!     let mut stripped_tracks = Vec::new();
//! 
//!     // 2
//!     for tag in tag_iterator {
//!         let mut tag = tag?;
//!         // 3
//!         if let Some(MatroskaSpec::TrackEntry) = tag.spec_tag {
//!             if let TagPosition::FullTag(_id, data) = &mut tag.tag {
//!                 if let TagData::Master(children) = data {
//!                     let is_audio_track = |tag: &mut (u64, TagData)| {
//!                         if MatroskaSpec::get_tag_id(&MatroskaSpec::TrackType) == tag.0 {
//!                             if let TagData::UnsignedInt(val) = tag.1 {
//!                                 return val != 2;
//!                             }
//!                         }
//!                         false
//!                     };
//! 
//!                     if children.iter_mut().any(is_audio_track) {
//!                         if let Some(track_number) = children.iter_mut().find(|c| c.0 == MatroskaSpec::get_tag_id(&MatroskaSpec::TrackNumber)) {
//!                             if let TagData::UnsignedInt(val) = track_number.1 {
//!                                 stripped_tracks.push(val);
//!                                 continue;
//!                             }
//!                         }
//!                     }
//!                 }
//!             }
//!         // 4
//!         } else if matches!(tag.spec_tag, Some(MatroskaSpec::Block)) || matches!(tag.spec_tag, Some(MatroskaSpec::SimpleBlock)) {
//!             if let TagPosition::FullTag(_id, tag) = tag.tag.clone() {
//!                 let block: Block = tag.try_into()?;
//!                 if stripped_tracks.iter().any(|t| *t == block.track) {
//!                     continue;
//!                 }
//!             }
//!         }
//!         // 5
//!         tag_writer.write(tag.tag)?;
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! In the above example, we (1) build our iterator and writer based on local file paths and declare useful local variables, (2) iterate over the tags in the webm file, (3) identify any track numbers that are not audio, store them in the `stripped_tracks` variable, and prevent writing the "TrackEntry" out, (4) avoid writing any block data for any tracks that are not audio, and (5) write remaining tags to the output destination.
//! 
//! __Notes__
//! * Notice the second parameter passed into the `WebmIterator::new()` function.  This parameter tells the decoder which `Master` tags should be read as `TagPosition::FullTag` tags rather than the standard `TagPosition::StartTag` and `TagPosition::EndTag` variants.  This greatly simplifies our iteration loop logic as we don't have to maintain an internal buffer for the "TrackEntry" tag that we are interested in processing.
//! 

use ebml_iterable::{TagIterator, TagWriter};

mod errors;
pub mod matroska_spec;

use matroska_spec::MatroskaSpec;

///
/// Alias for [`ebml_iterable::TagIterator`] using [`MatroskaSpec`] as the generic type. 
/// 
/// This implements Rust's standard [`Iterator`] trait. The struct can be created with the `new` function on any source that implements the [`std::io::Read`] trait. The iterator outputs `SpecTag` objects containing the type of Matroska tag and the tag data. See the [ebml-iterable](https://crates.io/crates/ebml_iterable) docs for more information if needed.
/// 
/// Note: The `with_capacity` method can be used to construct a `WebmIterator` with a specified default buffer size.  This is only useful as a microoptimization to memory management if you know the maximum tag size of the file you're reading.
/// 
pub type WebmIterator<R> = TagIterator<R, MatroskaSpec>;

///
/// Alias for [`ebml_iterable::TagWriter`]. 
/// 
/// This can be used to write webm files from tag data. This struct can be created with the `new` function on any source that implements the [`std::io::Write`] trait. See the [ebml-iterable](https://crates.io/crates/ebml_iterable) docs for more information if needed.
/// 
pub type WebmWriter<W> = TagWriter<W>;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::tags::{TagPosition, TagData};
    use super::WebmWriter;
    use super::WebmIterator;

    #[test]
    fn basic_tag_stream_write_and_iterate() {
        let tags: Vec<TagPosition> = vec![
            TagPosition::StartTag(0x1a45dfa3),
            TagPosition::StartTag(0x18538067),
            TagPosition::FullTag(0x83, TagData::UnsignedInt(0x01)),
            TagPosition::EndTag(0x18538067),
            TagPosition::FullTag(0x1f43b675, TagData::Master(vec![
                (0x97, TagData::UnsignedInt(0x02)),
            ])),
            TagPosition::EndTag(0x1a45dfa3),
        ];

        let mut dest = Cursor::new(Vec::new());
        let mut writer = WebmWriter::new(&mut dest);

        for tag in tags {
            writer.write(tag).expect("Test shouldn't error");
        }

        println!("dest {:?}", dest);

        let mut src = Cursor::new(dest.get_ref().to_vec());
        let reader = WebmIterator::new(&mut src, &[]);
        let tags: Vec<TagPosition> = reader.map(|i| i.unwrap().tag).collect();

        println!("tags {:?}", tags);

        assert_eq!(TagPosition::StartTag(0x1a45dfa3), tags[0]);
        assert_eq!(TagPosition::StartTag(0x18538067), tags[1]);
        assert_eq!(TagPosition::FullTag(0x83, TagData::UnsignedInt(0x01)), tags[2]);
        assert_eq!(TagPosition::EndTag(0x18538067), tags[3]);
        assert_eq!(TagPosition::StartTag(0x1f43b675), tags[4]);
        assert_eq!(TagPosition::FullTag(0x97, TagData::UnsignedInt(0x02)), tags[5]);
        assert_eq!(TagPosition::EndTag(0x1f43b675), tags[6]);
        assert_eq!(TagPosition::EndTag(0x1a45dfa3), tags[7]);
    }
}
