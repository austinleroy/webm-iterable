This crate was built to ease parsing files encoded in a Matroska container, such as [WebMs][webm] or
[MKVs][mkv].

```Cargo.toml
[dependencies]
webm-iterable = "0.1.0"
```

# Usage

The `WebmIterator` type is an alias for [ebml-iterable][ebml-iterable]'s `TagIterator` using `MatroskaSpec` as the generic type, and implements Rust's standard [Iterator][rust-iterator] trait. This struct can be created with the `new` function on any source that implements the standard [Read][rust-read] trait. The iterator outputs `SpecTag` objects containing the type of Matroska tag and the tag data.

> Note: The `with_capacity` method can be used to construct a `WebmIterator` with a specified default buffer size.  This is only useful as a microoptimization to memory management if you know the maximum tag size of the file you're reading.

The data in the `EbmlTag` property can then be modified as desired (encryption, compression, etc.) and reencoded using the `WebmWriter` type.  `WebmWriter` simply wraps [ebml-iterable][ebml-iterable]'s `TagWriter`. This struct can be created with the `new` function on any source that implements the standard [Write][rust-write] trait.

See the [ebml-iterable][ebml-iterable] docs for more information on `EbmlTag`, `DataTag`, and `DataTagType` if needed.

## Matroska-specific types

This crate provides two additional subtypes of `DataTagType` for ease of use:

  * [`Block`][mkv-block]
  * [`SimpleBlock`][mkv-sblock]

### Block

```rs
pub struct Block {
    pub payload: Vec<u8>,
    pub track: u64,
    pub value: i16,

    pub invisible: bool,
    pub lacing: BlockLacing,
}
```

These properties are specific to the [Block][mkv-block] element as defined by [Matroska][mkv].  The `Block` struct implements `TryFrom<DataTagType>` and `Into<DataTagType>` to simplify coercion to and from regular `DataTagType::Binary` values.

### SimpleBlock

```rs
pub struct SimpleBlock {
    pub block: Block,
    pub discardable: bool,
    pub keyframe: bool,
}
```

These properties are specific to the [SimpleBlock][mkv-sblock] element as defined by [Matroska][mkv].  The `SipleBlock` struct also implements `TryFrom<DataTagType>` and `Into<DataTagType>` to simplify coercion to and from regular `DataTagType::Binary` values.

# Examples

This example reads a media file into memory and decodes it.

```rs
use std::fs::File;
use webm_iterable::WebmIterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut src = File::open("media/test.webm").unwrap();
    let tag_iterator = WebmIterator::new(&mut src, &[]);

    for tag in tag_iterator {
        println!("[{:?}]", tag?.spec_type);
    }

    Ok(())
}
```

This example does the same thing, but keeps track of the number of times each tag appears in the file.

```rs
use std::fs::File;
use std::collections::HashMap;
use webm_iterable::WebmIterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut src = File::open("media/test.webm").unwrap();
    let tag_iterator = WebmIterator::new(&mut src, &[]);
    let mut tag_counts = HashMap::new();

    for tag in tag_iterator {
        let count = tag_counts.entry(tag?.spec_type).or_insert(0);
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
    matroska_spec::{MatroskaTag, MatroskaSpec, Block},
    tags::{EbmlTag, DataTagType, DataTag, TagSpec},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1
    let mut src = File::open("media/audiosample.webm").unwrap();
    let tag_iterator = WebmIterator::new(&mut src, &[MatroskaTag::TrackEntry]);
    let mut dest = File::create("media/audioout.webm").unwrap();
    let mut tag_writer = WebmWriter::new(&mut dest);
    let spec = MatroskaSpec {};
    let mut stripped_tracks = Vec::new();

    // 2
    for tag in tag_iterator {
        let mut tag = tag?;
        // 3
        if matches!(tag.spec_type, MatroskaTag::TrackEntry) {
            if let EbmlTag::FullTag(data) = &mut tag.tag {
                if let DataTagType::Master(children) = &mut data.data_type {
                    let is_audio_track = |tag: &mut DataTag| {
                        if matches!(spec.get_tag(tag.id), MatroskaTag::TrackType) {
                            if let DataTagType::UnsignedInt(val) = tag.data_type {
                                return val != 2;
                            }
                        }
                        false
                    };

                    if children.iter_mut().any(is_audio_track) {
                        if let Some(track_number) = children.iter_mut().find(|c| matches!(spec.get_tag(c.id), MatroskaTag::TrackNumber)) {
                            if let DataTagType::UnsignedInt(val) = track_number.data_type {
                                stripped_tracks.push(val);
                                continue;
                            }
                        }
                    }
                }
            }
        // 4
        } else if matches!(tag.spec_type, MatroskaTag::Block) || matches!(tag.spec_type, MatroskaTag::SimpleBlock) {
            if let EbmlTag::FullTag(tag) = tag.tag.clone() {
                let block: Block = tag.data_type.try_into()?;
                if stripped_tracks.iter().any(|t| *t == block.track) {
                    continue;
                }
            }
        }
        // 5
        tag_writer.write(tag.tag)?;
    }
    
    Ok(())
}
```

In the above example, we (1) build our iterator and writer based on local file paths and declare useful local variables, (2) iterate over the tags in the webm file, (3) identify any track numbers that are not audio, store them in the `stripped_tracks` variable, and prevent writing the "TrackEntry" out, (4) avoid writing any block data for any tracks that are not audio, and (5) write remaining tags to the output destination.

> __Notes__
> 
> * Notice the second parameter passed into the `WebmIterator::new()` function.  This parameter tells the decoder which `Master` tags should be read as `EbmlTag::FullTag` tags rather than the standard `EbmlTag::StartTag` and `EbmlTag::EndTag` variants.  This greatly simplifies our iteration loop logic as we don't have to maintain an internal buffer for the "TrackEntry" tag that we are interested in processing.
>


# State of this project

Parsing and writing complete files should both work.  Streaming isn't supported yet, but may be an option in the future. If something is broken, please create [an issue][new-issue].

Any additional feature requests can also be submitted as [an issue][new-issue].

# Author

[Austin Blake](https://github.com/austinleroy)

[EBML]: http://ebml.sourceforge.net/
[webm]: https://www.webmproject.org/
[mkv]: http://www.matroska.org/technical/specs/index.html
[mkv-block]: https://www.matroska.org/technical/specs/index.html#block_structure
[mkv-sblock]: https://www.matroska.org/technical/specs/index.html#simpleblock_structure
[rust-iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[rust-read]: https://doc.rust-lang.org/std/io/trait.Read.html
[rust-write]: https://doc.rust-lang.org/std/io/trait.Write.html
[ebml-iterable]: https://crates.io/crates/ebml-iterable
[new-issue]: https://github.com/austinleroy/webm-iterable/issues
