//!
//! Provides the [`MatroskaSpec`] enum, which implements [`EbmlSpecification`] and [`EbmlTag`].
//!
//! This is used in conjuction with the [ebml_iterable](https://crates.io/crates/ebml_iterable) library to be able to read and write Matroska formatted files based on raw tag data. Additionally, this module provides the [`Block`] and [`SimpleBlock`] structs, which provide an easy way to work with block data.  These can easily be converted to and from the regular enum variants using `into()` and `try_from()` to make working with the iterator stream easier.
//!

mod block;
mod simple_block;

pub use block::{Block, BlockLacing};
pub use simple_block::SimpleBlock;

pub use ebml_iterable::specs::{EbmlSpecification, EbmlTag, Master, TagDataType};
use ebml_iterable::specs::easy_ebml;

easy_ebml! {
    ///
    /// The Matroska specification
    ///
    /// Variants are all of the different tag types defined by the Matroska spec.
    ///
    #[derive(Clone, PartialEq, Debug)]
    pub enum MatroskaSpec {

        // Global Elements, automatically detected as global
        Crc32:  Binary  = 0xbf,
        Void:   Binary  = 0xec,

        // EBML Header
        Ebml:                                           Master      = 0x1a45dfa3,
        Ebml/EbmlVersion:                               UnsignedInt = 0x4286,
        Ebml/EbmlReadVersion:                           UnsignedInt = 0x42f7,
        Ebml/EbmlMaxIdLength:                           UnsignedInt = 0x42f2,
        Ebml/EbmlMaxSizeLength:                         UnsignedInt = 0x42f3,
        Ebml/DocType:                                   Utf8        = 0x4282,
        Ebml/DocTypeVersion:                            UnsignedInt = 0x4287,
        Ebml/DocTypeReadVersion:                        UnsignedInt = 0x4285,
        Ebml/DocTypeExtension:                          Master      = 0x4281,
        Ebml/DocTypeExtension/DocTypeExtensionName:     Utf8        = 0x4283,
        Ebml/DocTypeExtension/DocTypeExtensionVersion:  UnsignedInt = 0x4284,

        // MKV of spec
        Segment : Master = 0x18538067,

        Segment/Attachments : Master = 0x1941A469,
        Segment/Attachments/AttachedFile : Master = 0x61A7,
        Segment/Attachments/AttachedFile/FileData : Binary = 0x465C,
        Segment/Attachments/AttachedFile/FileDescription : Utf8 = 0x467E,
        Segment/Attachments/AttachedFile/FileMimeType : Utf8 = 0x4660,
        Segment/Attachments/AttachedFile/FileName : Utf8 = 0x466E,
        Segment/Attachments/AttachedFile/FileReferral : Binary = 0x4675,
        Segment/Attachments/AttachedFile/FileUID : UnsignedInt = 0x46AE,
        Segment/Attachments/AttachedFile/FileUsedEndTime : UnsignedInt = 0x4662,
        Segment/Attachments/AttachedFile/FileUsedStartTime : UnsignedInt = 0x4661,

        Segment/Chapters : Master = 0x1043A770,
        Segment/Chapters/EditionEntry : Master = 0x45B9,
        Segment/Chapters/EditionEntry/ChapterAtom : Master = 0xB6,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapProcess : Master = 0x6944,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapProcess/ChapProcessCodecID : UnsignedInt = 0x6955,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapProcess/ChapProcessCommand : Master = 0x6911,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapProcess/ChapProcessCommand/ChapProcessData : Binary = 0x6933,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapProcess/ChapProcessCommand/ChapProcessTime : UnsignedInt = 0x6922,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapProcess/ChapProcessPrivate : Binary = 0x450D,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterDisplay : Master = 0x80,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterDisplay/ChapCountry : Utf8 = 0x437E,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterDisplay/ChapLanguage : Utf8 = 0x437C,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterDisplay/ChapLanguageIETF : Utf8 = 0x437D,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterDisplay/ChapString : Utf8 = 0x85,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterFlagEnabled : UnsignedInt = 0x4598,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterFlagHidden : UnsignedInt = 0x98,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterPhysicalEquiv : UnsignedInt = 0x63C3,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterSegmentEditionUID : UnsignedInt = 0x6EBC,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterSegmentUID : Binary = 0x6E67,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterStringUID : Utf8 = 0x5654,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterTimeEnd : UnsignedInt = 0x92,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterTimeStart : UnsignedInt = 0x91,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterTrack : Master = 0x8F,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterTrack/ChapterTrackUID : UnsignedInt = 0x89,
        Segment/Chapters/EditionEntry/ChapterAtom/ChapterUID : UnsignedInt = 0x73C4,
        Segment/Chapters/EditionEntry/EditionFlagDefault : UnsignedInt = 0x45DB,
        Segment/Chapters/EditionEntry/EditionFlagHidden : UnsignedInt = 0x45BD,
        Segment/Chapters/EditionEntry/EditionFlagOrdered : UnsignedInt = 0x45DD,
        Segment/Chapters/EditionEntry/EditionUID : UnsignedInt = 0x45BC,

        Segment/Cluster : Master = 0x1F43B675,
        Segment/Cluster/BlockGroup : Master = 0xA0,
        Segment/Cluster/BlockGroup/Block : Binary = 0xA1,
        Segment/Cluster/BlockGroup/BlockAdditions : Master = 0x75A1,
        Segment/Cluster/BlockGroup/BlockAdditions/BlockMore : Master = 0xA6,
        Segment/Cluster/BlockGroup/BlockAdditions/BlockMore/BlockAddID : UnsignedInt = 0xEE,
        Segment/Cluster/BlockGroup/BlockAdditions/BlockMore/BlockAdditional : Binary = 0xA5,
        Segment/Cluster/BlockGroup/BlockDuration : UnsignedInt = 0x9B,
        Segment/Cluster/BlockGroup/BlockVirtual : Binary = 0xA2,
        Segment/Cluster/BlockGroup/CodecState : Binary = 0xA4,
        Segment/Cluster/BlockGroup/DiscardPadding : Integer = 0x75A2,
        Segment/Cluster/BlockGroup/ReferenceBlock : Integer = 0xFB,
        Segment/Cluster/BlockGroup/ReferenceFrame : Master = 0xC8,
        Segment/Cluster/BlockGroup/ReferenceFrame/ReferenceOffset : UnsignedInt = 0xC9,
        Segment/Cluster/BlockGroup/ReferenceFrame/ReferenceTimestamp : UnsignedInt = 0xCA,
        Segment/Cluster/BlockGroup/ReferencePriority : UnsignedInt = 0xFA,
        Segment/Cluster/BlockGroup/ReferenceVirtual : Integer = 0xFD,
        Segment/Cluster/BlockGroup/Slices : Master = 0x8E,
        Segment/Cluster/BlockGroup/Slices/TimeSlice : Master = 0xE8,
        Segment/Cluster/BlockGroup/Slices/TimeSlice/BlockAdditionID : UnsignedInt = 0xCB,
        Segment/Cluster/BlockGroup/Slices/TimeSlice/Delay : UnsignedInt = 0xCE,
        Segment/Cluster/BlockGroup/Slices/TimeSlice/FrameNumber : UnsignedInt = 0xCD,
        Segment/Cluster/BlockGroup/Slices/TimeSlice/LaceNumber : UnsignedInt = 0xCC,
        Segment/Cluster/BlockGroup/Slices/TimeSlice/SliceDuration : UnsignedInt = 0xCF,
        Segment/Cluster/EncryptedBlock : Binary = 0xAF,
        Segment/Cluster/Position : UnsignedInt = 0xA7,
        Segment/Cluster/PrevSize : UnsignedInt = 0xAB,
        Segment/Cluster/SilentTracks : Master = 0x5854,
        Segment/Cluster/SilentTracks/SilentTrackNumber : UnsignedInt = 0x58D7,
        Segment/Cluster/SimpleBlock : Binary = 0xA3,
        Segment/Cluster/Timestamp : UnsignedInt = 0xE7,

        Segment/Cues : Master = 0x1C53BB6B,
        Segment/Cues/CuePoint : Master = 0xBB,
        Segment/Cues/CuePoint/CueTime : UnsignedInt = 0xB3,
        Segment/Cues/CuePoint/CueTrackPositions : Master = 0xB7,
        Segment/Cues/CuePoint/CueTrackPositions/CueBlockNumber : UnsignedInt = 0x5378,
        Segment/Cues/CuePoint/CueTrackPositions/CueClusterPosition : UnsignedInt = 0xF1,
        Segment/Cues/CuePoint/CueTrackPositions/CueCodecState : UnsignedInt = 0xEA,
        Segment/Cues/CuePoint/CueTrackPositions/CueDuration : UnsignedInt = 0xB2,
        Segment/Cues/CuePoint/CueTrackPositions/CueReference : Master = 0xDB,
        Segment/Cues/CuePoint/CueTrackPositions/CueReference/CueRefCluster : UnsignedInt = 0x97,
        Segment/Cues/CuePoint/CueTrackPositions/CueReference/CueRefCodecState : UnsignedInt = 0xEB,
        Segment/Cues/CuePoint/CueTrackPositions/CueReference/CueRefNumber : UnsignedInt = 0x535F,
        Segment/Cues/CuePoint/CueTrackPositions/CueReference/CueRefTime : UnsignedInt = 0x96,
        Segment/Cues/CuePoint/CueTrackPositions/CueRelativePosition : UnsignedInt = 0xF0,
        Segment/Cues/CuePoint/CueTrackPositions/CueTrack : UnsignedInt = 0xF7,

        Segment/Info : Master = 0x1549A966,
        Segment/Info/ChapterTranslate : Master = 0x6924,
        Segment/Info/ChapterTranslate/ChapterTranslateCodec : UnsignedInt = 0x69BF,
        Segment/Info/ChapterTranslate/ChapterTranslateEditionUID : UnsignedInt = 0x69FC,
        Segment/Info/ChapterTranslate/ChapterTranslateID : Binary = 0x69A5,
        Segment/Info/DateUTC : Integer = 0x4461,
        Segment/Info/Duration : Float = 0x4489,
        Segment/Info/MuxingApp : Utf8 = 0x4D80,
        Segment/Info/NextFilename : Utf8 = 0x3E83BB,
        Segment/Info/NextUID : Binary = 0x3EB923,
        Segment/Info/PrevFilename : Utf8 = 0x3C83AB,
        Segment/Info/PrevUID : Binary = 0x3CB923,
        Segment/Info/SegmentFamily : Binary = 0x4444,
        Segment/Info/SegmentFilename : Utf8 = 0x7384,
        Segment/Info/SegmentUID : Binary = 0x73A4,
        Segment/Info/TimestampScale : UnsignedInt = 0x2AD7B1,
        Segment/Info/Title : Utf8 = 0x7BA9,
        Segment/Info/WritingApp : Utf8 = 0x5741,

        Segment/SeekHead : Master = 0x114D9B74,
        Segment/SeekHead/Seek : Master = 0x4DBB,
        Segment/SeekHead/Seek/SeekID : Binary = 0x53AB,
        Segment/SeekHead/Seek/SeekPosition : UnsignedInt = 0x53AC,

        Segment/Tags : Master = 0x1254C367,
        Segment/Tags/Tag : Master = 0x7373,
        Segment/Tags/Tag/SimpleTag : Master = 0x67C8,
        Segment/Tags/Tag/SimpleTag/TagBinary : Binary = 0x4485,
        Segment/Tags/Tag/SimpleTag/TagDefault : UnsignedInt = 0x4484,
        Segment/Tags/Tag/SimpleTag/TagDefaultBogus : UnsignedInt = 0x44B4,
        Segment/Tags/Tag/SimpleTag/TagLanguage : Utf8 = 0x447A,
        Segment/Tags/Tag/SimpleTag/TagLanguageIETF : Utf8 = 0x447B,
        Segment/Tags/Tag/SimpleTag/TagName : Utf8 = 0x45A3,
        Segment/Tags/Tag/SimpleTag/TagString : Utf8 = 0x4487,
        Segment/Tags/Tag/Targets : Master = 0x63C0,
        Segment/Tags/Tag/Targets/TagAttachmentUID : UnsignedInt = 0x63C6,
        Segment/Tags/Tag/Targets/TagChapterUID : UnsignedInt = 0x63C4,
        Segment/Tags/Tag/Targets/TagEditionUID : UnsignedInt = 0x63C9,
        Segment/Tags/Tag/Targets/TagTrackUID : UnsignedInt = 0x63C5,
        Segment/Tags/Tag/Targets/TargetType : Utf8 = 0x63CA,
        Segment/Tags/Tag/Targets/TargetTypeValue : UnsignedInt = 0x68CA,

        Segment/Tracks : Master = 0x1654AE6B,
        Segment/Tracks/TrackEntry : Master = 0xAE,
        Segment/Tracks/TrackEntry/AttachmentLink : UnsignedInt = 0x7446,
        Segment/Tracks/TrackEntry/Audio : Master = 0xE1,
        Segment/Tracks/TrackEntry/Audio/BitDepth : UnsignedInt = 0x6264,
        Segment/Tracks/TrackEntry/Audio/ChannelPositions : Binary = 0x7D7B,
        Segment/Tracks/TrackEntry/Audio/Channels : UnsignedInt = 0x9F,
        Segment/Tracks/TrackEntry/Audio/OutputSamplingFrequency : Float = 0x78B5,
        Segment/Tracks/TrackEntry/Audio/SamplingFrequency : Float = 0xB5,
        Segment/Tracks/TrackEntry/BlockAdditionMapping : Master = 0x41E4,
        Segment/Tracks/TrackEntry/BlockAdditionMapping/BlockAddIDExtraData : Binary = 0x41ED,
        Segment/Tracks/TrackEntry/BlockAdditionMapping/BlockAddIDName : Utf8 = 0x41A4,
        Segment/Tracks/TrackEntry/BlockAdditionMapping/BlockAddIDType : UnsignedInt = 0x41E7,
        Segment/Tracks/TrackEntry/BlockAdditionMapping/BlockAddIDValue : UnsignedInt = 0x41F0,
        Segment/Tracks/TrackEntry/CodecDecodeAll : UnsignedInt = 0xAA,
        Segment/Tracks/TrackEntry/CodecDelay : UnsignedInt = 0x56AA,
        Segment/Tracks/TrackEntry/CodecDownloadURL : Utf8 = 0x26B240,
        Segment/Tracks/TrackEntry/CodecID : Utf8 = 0x86,
        Segment/Tracks/TrackEntry/CodecInfoURL : Utf8 = 0x3B4040,
        Segment/Tracks/TrackEntry/CodecName : Utf8 = 0x258688,
        Segment/Tracks/TrackEntry/CodecPrivate : Binary = 0x63A2,
        Segment/Tracks/TrackEntry/CodecSettings : Utf8 = 0x3A9697,
        Segment/Tracks/TrackEntry/ContentEncodings : Master = 0x6D80,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding : Master = 0x6240,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentCompression : Master = 0x5034,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentCompression/ContentCompAlgo : UnsignedInt = 0x4254,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentCompression/ContentCompSettings : Binary = 0x4255,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncodingOrder : UnsignedInt = 0x5031,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncodingScope : UnsignedInt = 0x5032,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncodingType : UnsignedInt = 0x5033,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncryption : Master = 0x5035,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncryption/ContentEncAESSettings : Master = 0x47E7,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncryption/ContentEncAESSettings/AESSettingsCipherMode : UnsignedInt = 0x47E8,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncryption/ContentEncAlgo : UnsignedInt = 0x47E1,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncryption/ContentEncKeyID : Binary = 0x47E2,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncryption/ContentSigAlgo : UnsignedInt = 0x47E5,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncryption/ContentSigHashAlgo : UnsignedInt = 0x47E6,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncryption/ContentSigKeyID : Binary = 0x47E4,
        Segment/Tracks/TrackEntry/ContentEncodings/ContentEncoding/ContentEncryption/ContentSignature : Binary = 0x47E3,
        Segment/Tracks/TrackEntry/DefaultDecodedFieldDuration : UnsignedInt = 0x234E7A,
        Segment/Tracks/TrackEntry/DefaultDuration : UnsignedInt = 0x23E383,
        Segment/Tracks/TrackEntry/FlagCommentary : UnsignedInt = 0x55AF,
        Segment/Tracks/TrackEntry/FlagDefault : UnsignedInt = 0x88,
        Segment/Tracks/TrackEntry/FlagEnabled : UnsignedInt = 0xB9,
        Segment/Tracks/TrackEntry/FlagForced : UnsignedInt = 0x55AA,
        Segment/Tracks/TrackEntry/FlagHearingImpaired : UnsignedInt = 0x55AB,
        Segment/Tracks/TrackEntry/FlagLacing : UnsignedInt = 0x9C,
        Segment/Tracks/TrackEntry/FlagOriginal : UnsignedInt = 0x55AE,
        Segment/Tracks/TrackEntry/FlagTextDescriptions : UnsignedInt = 0x55AD,
        Segment/Tracks/TrackEntry/FlagVisualImpaired : UnsignedInt = 0x55AC,
        Segment/Tracks/TrackEntry/Language : Utf8 = 0x22B59C,
        Segment/Tracks/TrackEntry/LanguageIETF : Utf8 = 0x22B59D,
        Segment/Tracks/TrackEntry/MaxBlockAdditionID : UnsignedInt = 0x55EE,
        Segment/Tracks/TrackEntry/MaxCache : UnsignedInt = 0x6DF8,
        Segment/Tracks/TrackEntry/MinCache : UnsignedInt = 0x6DE7,
        Segment/Tracks/TrackEntry/Name : Utf8 = 0x536E,
        Segment/Tracks/TrackEntry/SeekPreRoll : UnsignedInt = 0x56BB,
        Segment/Tracks/TrackEntry/TrackNumber : UnsignedInt = 0xD7,
        Segment/Tracks/TrackEntry/TrackOffset : Integer = 0x537F,
        Segment/Tracks/TrackEntry/TrackOperation : Master = 0xE2,
        Segment/Tracks/TrackEntry/TrackOperation/TrackCombinePlanes : Master = 0xE3,
        Segment/Tracks/TrackEntry/TrackOperation/TrackCombinePlanes/TrackPlane : Master = 0xE4,
        Segment/Tracks/TrackEntry/TrackOperation/TrackCombinePlanes/TrackPlane/TrackPlaneType : UnsignedInt = 0xE6,
        Segment/Tracks/TrackEntry/TrackOperation/TrackCombinePlanes/TrackPlane/TrackPlaneUID : UnsignedInt = 0xE5,
        Segment/Tracks/TrackEntry/TrackOperation/TrackJoinBlocks : Master = 0xE9,
        Segment/Tracks/TrackEntry/TrackOperation/TrackJoinBlocks/TrackJoinUID : UnsignedInt = 0xED,
        Segment/Tracks/TrackEntry/TrackOverlay : UnsignedInt = 0x6FAB,
        Segment/Tracks/TrackEntry/TrackTimestampScale : Float = 0x23314F,
        Segment/Tracks/TrackEntry/TrackTranslate : Master = 0x6624,
        Segment/Tracks/TrackEntry/TrackTranslate/TrackTranslateCodec : UnsignedInt = 0x66BF,
        Segment/Tracks/TrackEntry/TrackTranslate/TrackTranslateEditionUID : UnsignedInt = 0x66FC,
        Segment/Tracks/TrackEntry/TrackTranslate/TrackTranslateTrackID : Binary = 0x66A5,
        Segment/Tracks/TrackEntry/TrackType : UnsignedInt = 0x83,
        Segment/Tracks/TrackEntry/TrackUID : UnsignedInt = 0x73C5,
        Segment/Tracks/TrackEntry/TrickMasterTrackSegmentUID : Binary = 0xC4,
        Segment/Tracks/TrackEntry/TrickMasterTrackUID : UnsignedInt = 0xC7,
        Segment/Tracks/TrackEntry/TrickTrackFlag : UnsignedInt = 0xC6,
        Segment/Tracks/TrackEntry/TrickTrackSegmentUID : Binary = 0xC1,
        Segment/Tracks/TrackEntry/TrickTrackUID : UnsignedInt = 0xC0,
        Segment/Tracks/TrackEntry/Video : Master = 0xE0,
        Segment/Tracks/TrackEntry/Video/AlphaMode : UnsignedInt = 0x53C0,
        Segment/Tracks/TrackEntry/Video/AspectRatioType : UnsignedInt = 0x54B3,
        Segment/Tracks/TrackEntry/Video/Colour : Master = 0x55B0,
        Segment/Tracks/TrackEntry/Video/Colour/BitsPerChannel : UnsignedInt = 0x55B2,
        Segment/Tracks/TrackEntry/Video/Colour/CbSubsamplingHorz : UnsignedInt = 0x55B5,
        Segment/Tracks/TrackEntry/Video/Colour/CbSubsamplingVert : UnsignedInt = 0x55B6,
        Segment/Tracks/TrackEntry/Video/Colour/ChromaSitingHorz : UnsignedInt = 0x55B7,
        Segment/Tracks/TrackEntry/Video/Colour/ChromaSitingVert : UnsignedInt = 0x55B8,
        Segment/Tracks/TrackEntry/Video/Colour/ChromaSubsamplingHorz : UnsignedInt = 0x55B3,
        Segment/Tracks/TrackEntry/Video/Colour/ChromaSubsamplingVert : UnsignedInt = 0x55B4,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata : Master = 0x55D0,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata/LuminanceMax : Float = 0x55D9,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata/LuminanceMin : Float = 0x55DA,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata/PrimaryBChromaticityX : Float = 0x55D5,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata/PrimaryBChromaticityY : Float = 0x55D6,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata/PrimaryGChromaticityX : Float = 0x55D3,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata/PrimaryGChromaticityY : Float = 0x55D4,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata/PrimaryRChromaticityX : Float = 0x55D1,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata/PrimaryRChromaticityY : Float = 0x55D2,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata/WhitePointChromaticityX : Float = 0x55D7,
        Segment/Tracks/TrackEntry/Video/Colour/MasteringMetadata/WhitePointChromaticityY : Float = 0x55D8,
        Segment/Tracks/TrackEntry/Video/Colour/MatrixCoefficients : UnsignedInt = 0x55B1,
        Segment/Tracks/TrackEntry/Video/Colour/MaxCLL : UnsignedInt = 0x55BC,
        Segment/Tracks/TrackEntry/Video/Colour/MaxFALL : UnsignedInt = 0x55BD,
        Segment/Tracks/TrackEntry/Video/Colour/Primaries : UnsignedInt = 0x55BB,
        Segment/Tracks/TrackEntry/Video/Colour/Range : UnsignedInt = 0x55B9,
        Segment/Tracks/TrackEntry/Video/Colour/TransferCharacteristics : UnsignedInt = 0x55BA,
        Segment/Tracks/TrackEntry/Video/DisplayHeight : UnsignedInt = 0x54BA,
        Segment/Tracks/TrackEntry/Video/DisplayUnit : UnsignedInt = 0x54B2,
        Segment/Tracks/TrackEntry/Video/DisplayWidth : UnsignedInt = 0x54B0,
        Segment/Tracks/TrackEntry/Video/FieldOrder : UnsignedInt = 0x9D,
        Segment/Tracks/TrackEntry/Video/FlagInterlaced : UnsignedInt = 0x9A,
        Segment/Tracks/TrackEntry/Video/FrameRate : Float = 0x2383E3,
        Segment/Tracks/TrackEntry/Video/GammaValue : Float = 0x2FB523,
        Segment/Tracks/TrackEntry/Video/OldStereoMode : UnsignedInt = 0x53B9,
        Segment/Tracks/TrackEntry/Video/PixelCropBottom : UnsignedInt = 0x54AA,
        Segment/Tracks/TrackEntry/Video/PixelCropLeft : UnsignedInt = 0x54CC,
        Segment/Tracks/TrackEntry/Video/PixelCropRight : UnsignedInt = 0x54DD,
        Segment/Tracks/TrackEntry/Video/PixelCropTop : UnsignedInt = 0x54BB,
        Segment/Tracks/TrackEntry/Video/PixelHeight : UnsignedInt = 0xBA,
        Segment/Tracks/TrackEntry/Video/PixelWidth : UnsignedInt = 0xB0,
        Segment/Tracks/TrackEntry/Video/Projection : Master = 0x7670,
        Segment/Tracks/TrackEntry/Video/Projection/ProjectionPosePitch : Float = 0x7674,
        Segment/Tracks/TrackEntry/Video/Projection/ProjectionPoseRoll : Float = 0x7675,
        Segment/Tracks/TrackEntry/Video/Projection/ProjectionPoseYaw : Float = 0x7673,
        Segment/Tracks/TrackEntry/Video/Projection/ProjectionPrivate : Binary = 0x7672,
        Segment/Tracks/TrackEntry/Video/Projection/ProjectionType : UnsignedInt = 0x7671,
        Segment/Tracks/TrackEntry/Video/StereoMode : UnsignedInt = 0x53B8,
        Segment/Tracks/TrackEntry/Video/UncompressedFourCC : Binary = 0x2EB524,
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;
    use hyper::Client;
    use hyper_tls::HttpsConnector;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct EBMLSpec {
        #[serde(rename = "$value")]
        elements: Vec<EBMLElement>
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct EBMLElement {
        path: String,
        id: String,
        r#type: String,
    }

    #[ignore]
    #[tokio::test]
    async fn print_spec() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = "https://raw.githubusercontent.com/ietf-wg-cellar/matroska-specification/master/ebml_matroska.xml";
        let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
        let resp = client.get(url.parse()?).await?;
        let bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let str = from_utf8(&bytes)?;
        let spec: EBMLSpec = serde_xml_rs::from_str(str)?;

        let mut lines = vec![];
        for EBMLElement {  path, id, r#type } in spec.elements {
            let ty = match r#type.as_str() {
                "master" => "Master",
                "uinteger" => "UnsignedInt",
                "integer" | "date" => "Integer",
                "utf-8" | "string" => "Utf8",
                "binary" => "Binary",
                "float" => "Float",
                _ => unreachable!("unknown type: {}", r#type)
            };
            let path = path.trim_start_matches("\\").replace("\\", "/").replace("+", "");
            lines.push(format!("{} : {} = {},", path, ty, id));
        }
        lines.sort();
        for line in lines {
            println!("{}", line);
        }
        Ok(())
    }
}
