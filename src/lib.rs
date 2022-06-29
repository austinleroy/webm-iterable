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
//!         println!("[{:?}]", tag?);
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
//! use webm_iterable::matroska_spec::EbmlTag;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut src = File::open("media/test.webm").unwrap();
//!     let tag_iterator = WebmIterator::new(&mut src, &[]);
//!     let mut tag_counts = HashMap::new();
//! 
//!     for tag in tag_iterator {
//!         let count = tag_counts.entry(tag?.get_id()).or_insert(0);
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
//!     matroska_spec::{MatroskaSpec, Master, Block, EbmlSpecification, EbmlTag},
//! };
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 1
//!     let mut src = File::open("media/audiosample.webm").unwrap();
//!     let tag_iterator = WebmIterator::new(&mut src, &[MatroskaSpec::TrackEntry(Master::Start)]);
//!     let mut dest = File::create("media/audioout.webm").unwrap();
//!     let mut tag_writer = WebmWriter::new(&mut dest);
//!     let mut stripped_tracks = Vec::new();
//! 
//!     // 2
//!     for tag in tag_iterator {
//!         let tag = tag?;
//!         match tag {
//!             // 3
//!             MatroskaSpec::TrackEntry(master) => {
//!                 let children = master.get_children();
//!                 let is_audio_track = |tag: &MatroskaSpec| {
//!                     if let MatroskaSpec::TrackType(val) = tag {
//!                         return *val != 2;
//!                     } else {
//!                         false
//!                     }
//!                 };
//! 
//!                 if children.iter().any(is_audio_track) {
//!                     let track_number_variant = children.iter().find(|c| matches!(c, MatroskaSpec::TrackNumber(_))).expect("should have a track number child");
//!                     let track_number = track_number_variant.as_unsigned_int().expect("TrackNumber is an unsigned int variant");
//!                     stripped_tracks.push(*track_number);
//!                 } else {
//!                     tag_writer.write(&MatroskaSpec::TrackEntry(Master::Full(children)))?;
//!                 }
//!             },
//!             // 4
//!             MatroskaSpec::Block(ref data) => {
//!                 let data: &[u8] = &data;
//!                 let block: Block = data.try_into()?;
//!                 if !stripped_tracks.iter().any(|t| *t == block.track) {
//!                     tag_writer.write(&tag)?;
//!                 }
//!             },
//!             MatroskaSpec::SimpleBlock(ref data) => {
//!                 let data: &[u8] = &data;
//!                 let block: Block = data.try_into()?;
//!                 if !stripped_tracks.iter().any(|t| *t == block.track) {
//!                     tag_writer.write(&tag)?;
//!                 }
//!             },
//!             // 5
//!             _ => {
//!                 tag_writer.write(&tag)?;
//!             }
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! In the above example, we (1) build our iterator and writer based on local file paths and declare useful local variables, (2) iterate over the tags in the webm file, (3) identify any tracks that are not audio and store their numbers in the `stripped_tracks` variable; if they are audio, we write the "TrackEntry" out, (4) only write block data for tracks that are audio, and (5) write all other tags to the output destination.
//! 
//! __Notes__
//! * Notice the second parameter passed into the `WebmIterator::new()` function.  This parameter tells the decoder which "master" tags should be read as [`Master::Full`][`crate::matroska_spec::Master::Full`] variants rather than the standard [`Master::Start`][`crate::matroska_spec::Master::Start`] and [`Master::End`][`crate::matroska_spec::Master::End`] variants.  This greatly simplifies our iteration loop logic as we don't have to maintain an internal buffer for the "TrackEntry" tags that we are interested in processing.
//! 

use ebml_iterable::{TagIterator, TagWriter};

pub mod errors;
pub mod matroska_spec;

use matroska_spec::MatroskaSpec;

///
/// Alias for [`ebml_iterable::TagIterator`] using [`MatroskaSpec`] as the generic type.
///
/// This implements Rust's standard [`Iterator`] trait. The struct can be created with the `new` function on any source that implements the [`std::io::Read`] trait. The iterator outputs [`MatroskaSpec`] variants containing the tag data. See the [ebml-iterable](https://crates.io/crates/ebml_iterable) docs for more information if needed.
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

    use super::matroska_spec::{MatroskaSpec, Master};
    use super::WebmWriter;
    use super::WebmIterator;

    #[test]
    fn basic_tag_stream_write_and_iterate() {
        let tags: Vec<MatroskaSpec> = vec![
            MatroskaSpec::Ebml(Master::Start),
            MatroskaSpec::Segment(Master::Start),
            MatroskaSpec::TrackType(0x01),
            MatroskaSpec::Segment(Master::End),
            MatroskaSpec::Cluster(Master::Full(vec![
                MatroskaSpec::CueRefCluster(0x02),
            ])),
            MatroskaSpec::Ebml(Master::End),
        ];

        let mut dest = Cursor::new(Vec::new());
        let mut writer = WebmWriter::new(&mut dest);

        for tag in tags {
            writer.write(&tag).expect("Test shouldn't error");
        }

        println!("dest {:?}", dest);

        let mut src = Cursor::new(dest.get_ref().to_vec());
        let reader = WebmIterator::new(&mut src, &[]);
        let tags: Vec<MatroskaSpec> = reader.map(|i| i.unwrap()).collect();

        println!("tags {:?}", tags);

        assert_eq!(MatroskaSpec::Ebml(Master::Start), tags[0]);
        assert_eq!(MatroskaSpec::Segment(Master::Start), tags[1]);
        assert_eq!(MatroskaSpec::TrackType(0x01), tags[2]);
        assert_eq!(MatroskaSpec::Segment(Master::End), tags[3]);
        assert_eq!(MatroskaSpec::Cluster(Master::Start), tags[4]);
        assert_eq!(MatroskaSpec::CueRefCluster(0x02), tags[5]);
        assert_eq!(MatroskaSpec::Cluster(Master::End), tags[6]);
        assert_eq!(MatroskaSpec::Ebml(Master::End), tags[7]);
    }
}
