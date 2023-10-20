This crate was built to ease parsing files encoded in a Matroska container, such as [WebMs][webm] or
[MKVs][mkv].

```Cargo.toml
[dependencies]
webm-iterable = "0.6.0"
```

# Usage

The `WebmIterator` type is an alias for [ebml-iterable][ebml-iterable]'s `TagIterator` using `MatroskaSpec` as the generic type, and implements Rust's standard [Iterator][rust-iterator] trait. This struct can be created with the `new` function on any source that implements the standard [Read][rust-read] trait. The iterator outputs `MatroskaSpec` variants containing the tag data.

> Note: The `with_capacity` method can be used to construct a `WebmIterator` with a specified default buffer size.  This is only useful as a microoptimization to memory management if you know the maximum tag size of the file you're reading.

The data in the tag can then be modified as desired (encryption, compression, etc.) and reencoded using the `WebmWriter` type.  `WebmWriter` simply wraps [ebml-iterable][ebml-iterable]'s `TagWriter`. This struct can be created with the `new` function on any source that implements the standard [Write][rust-write] trait.

See the [ebml-iterable][ebml-iterable] docs for more information on iterating over ebml data if needed.

## Matroska-specific types

This crate provides three additional structs for special matroska data tags:

  * [`Block`][mkv-block]
  * [`SimpleBlock`][mkv-sblock]
  * [`Frame`] (not a tag itself, but used within Blocks & Simple Blocks)

### Block

```rs
pub struct Block {
    pub track: u64,
    pub timestamp: i16,

    pub invisible: bool,
    pub lacing: BlockLacing,
}

impl Block {
    pub fn read_frame_data(&self) -> Result<Vec<Frame>, WebmCoercionError>
    pub fn raw_frame_data(&self) -> &[u8]
}
```

These properties are specific to the [Block][mkv-block] element as defined by [Matroska][mkv].  The `Block` struct implements `TryFrom<&MatroskaSpec>` and `Into<MatroskaSpec>` to simplify coercion to and from regular variants.

### SimpleBlock

```rs
pub struct SimpleBlock {
    pub track: u64,
    pub timestamp: i16,

    pub invisible: bool,
    pub lacing: Option<BlockLacing>,
    pub discardable: bool,
    pub keyframe: bool,
}

impl SimpleBlock {
    pub fn read_frame_data(&self) -> Result<Vec<Frame>, WebmCoercionError>
    pub fn raw_frame_data(&self) -> &[u8]
}
```

These properties are specific to the [SimpleBlock][mkv-sblock] element as defined by [Matroska][mkv].  The `SimpleBlock` struct also implements `TryFrom<&MatroskaSpec>` and `Into<MatroskaSpec>` to simplify coercion to and from regular variants.

# Examples

This example reads a media file into memory and decodes it.

```rs
use std::fs::File;
use webm_iterable::WebmIterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut src = File::open("media/test.webm").unwrap();
    let tag_iterator = WebmIterator::new(&mut src, &[]);

    for tag in tag_iterator {
        println!("[{:?}]", tag?);
    }

    Ok(())
}
```

This example does the same thing, but keeps track of the number of times each tag appears in the file.

```rs
use std::fs::File;
use std::collections::HashMap;
use webm_iterable::WebmIterator;
use webm_iterable::matroska_spec::EbmlTag;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut src = File::open("media/test.webm").unwrap();
    let tag_iterator = WebmIterator::new(&mut src, &[]);
    let mut tag_counts = HashMap::new();

    for tag in tag_iterator {
        let count = tag_counts.entry(tag?.get_id()).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", tag_counts);
    Ok(())
}
```

This example grabs the audio from a webm and stores the result in a new file.  The logic in this example is rather advanced - an explanation follows the code.

```rs
use std::fs::File;
use std::convert::TryInto;

use webm_iterable::{
    WebmIterator, 
    WebmWriter,
    matroska_spec::{MatroskaSpec, Master, Block, EbmlSpecification, EbmlTag},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1
    let mut src = File::open("media/audiosample.webm").unwrap();
    let tag_iterator = WebmIterator::new(&mut src, &[MatroskaSpec::TrackEntry(Master::Start)]);
    let mut dest = File::create("media/audioout.webm").unwrap();
    let mut tag_writer = WebmWriter::new(&mut dest);
    let mut stripped_tracks = Vec::new();

    // 2
    for tag in tag_iterator {
        let tag = tag?;
        match tag {
            // 3
            MatroskaSpec::TrackEntry(master) => {
                let children = master.get_children();
                let is_audio_track = |tag: &MatroskaSpec| {
                    if let MatroskaSpec::TrackType(val) = tag {
                        return *val != 2;
                    } else {
                        false
                    }
                };

                if children.iter().any(is_audio_track) {
                    let track_number_variant = children.iter().find(|c| matches!(c, MatroskaSpec::TrackNumber(_))).expect("should have a k number child");
                    let track_number = track_number_variant.as_unsigned_int().expect("TrackNumber is an unsigned int variant");
                    stripped_tracks.push(*track_number);
                } else {
                    tag_writer.write(&MatroskaSpec::TrackEntry(Master::Full(children)))?;
                }
            },
            // 4
            MatroskaSpec::Block(ref data) => {
                let block: Block = data.try_into()?;
                if !stripped_tracks.iter().any(|t| *t == block.track) {
                    tag_writer.write(&tag)?;
                }
            },
            MatroskaSpec::SimpleBlock(ref data) => {
                let block: Block = data.try_into()?;
                if !stripped_tracks.iter().any(|t| *t == block.track) {
                    tag_writer.write(&tag)?;
                }
            },
            // 5
            _ => {
                tag_writer.write(&tag)?;
            }
        }
    }
    
    Ok(())
}
```

In the above example, we (1) build our iterator and writer based on local file paths and declare useful local variables, (2) iterate over the tags in the webm file, (3) identify any tracks that are not audio and store their numbers in the `stripped_tracks` variable; if they are audio, we write the "TrackEntry" out, (4) only write block data for tracks that are audio, and (5) write all other tags to the output destination.

> __Notes__
> 
> * Notice the second parameter passed into the `WebmIterator::new()` function.  This parameter tells the decoder which "master" tags should be read as `Master::Full` variants rather than the standard `Master::Start` and `Master::End` variants.  This greatly simplifies our iteration loop logic as we don't have to maintain an internal buffer for the "TrackEntry" tags that we are interested in processing.
>


# State of this project

Parsing and writing complete files should both work.  Streaming (using tags of unknown size) should now also supported as of version 0.4.0. If something is broken, please create [an issue][new-issue].

Any additional feature requests can also be submitted as [an issue][new-issue].

# Author

[Austin Blake](https://github.com/austinleroy)

[webm]: https://www.webmproject.org/
[mkv]: http://www.matroska.org/technical/specs/index.html
[mkv-block]: https://www.matroska.org/technical/specs/index.html#block_structure
[mkv-sblock]: https://www.matroska.org/technical/specs/index.html#simpleblock_structure
[rust-iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[rust-read]: https://doc.rust-lang.org/std/io/trait.Read.html
[rust-write]: https://doc.rust-lang.org/std/io/trait.Write.html
[ebml-iterable]: https://crates.io/crates/ebml-iterable
[new-issue]: https://github.com/austinleroy/webm-iterable/issues
