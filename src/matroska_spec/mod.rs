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
use ebml_iterable::specs::{easy_ebml, ebml_specification};

///
/// The Matroska specification
///
/// Variants are all of the different tag types defined by the Matroska spec.
///
easy_ebml! {
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
        Segment: Master = 0x18538067,

        // Seek
        Segment/SeekHead: Master = 0x114d9b74,
        Segment/SeekHead/Seek: Master = 0x4dbb,
        Segment/SeekHead/Seek/SeekPosition: UnsignedInt = 0x53ac,
        Segment/SeekHead/Seek/SeekId: Binary = 0x53ab,
    }
}

///
/// The Matroska specification, as an enum.
///
/// Variants are all of the different tag types defined by the Matroska spec.
///
#[ebml_specification]
#[derive(Clone, PartialEq, Debug)]
pub enum MatroskaSpec2 {

    // Segment Info

    #[id(0x1549a966)]
    #[data_type(TagDataType::Master)]
    #[parent(Segment)]
    Info,

    #[id(0x73a4)]
    #[data_type(TagDataType::Binary)]
    #[parent(Info)]
    SegmentUid,

    #[id(0x7384)]
    #[data_type(TagDataType::Utf8)]
    #[parent(Info)]
    SegmentFilename,

    #[id(0x6e67)]
    #[data_type(TagDataType::Binary)]
    #[parent(Info)]
    ChapterSegmentUid,

    #[id(0x3cb923)]
    #[data_type(TagDataType::Binary)]
    #[parent(Info)]
    PrevUid,

    #[id(0x3c83ab)]
    #[data_type(TagDataType::Utf8)]
    #[parent(Info)]
    PrevFilename,

    #[id(0x3eb923)]
    #[data_type(TagDataType::Binary)]
    #[parent(Info)]
    NextUid,

    #[id(0x3e83bb)]
    #[data_type(TagDataType::Utf8)]
    #[parent(Info)]
    NextFilename,

    #[id(0x4444)]
    #[data_type(TagDataType::Binary)]
    #[parent(Info)]
    SegmentFamily,

    #[id(0x6924)]
    #[data_type(TagDataType::Master)]
    #[parent(Info)]
    ChapterTranslate,

    #[id(0x69fc)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(ChapterTranslate)]
    ChapterTranslateEditionUid,

    #[id(0x69bf)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(ChapterTranslate)]
    ChapterTranslateCodec,

    #[id(0x69a5)]
    #[data_type(TagDataType::Binary)]
    #[parent(ChapterTranslate)]
    ChapterTranslateId,

    #[id(0x2ad7b1)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(Info)]
    TimecodeScale,

    #[id(0x4489)]
    #[data_type(TagDataType::Float)]
    #[parent(Info)]
    Duration,

    #[id(0x4461)]
    #[data_type(TagDataType::Binary)]
    #[parent(Info)]
    DateUtc,

    #[id(0x7ba9)]
    #[data_type(TagDataType::Utf8)]
    #[parent(Info)]
    Title,

    #[id(0x4d80)]
    #[data_type(TagDataType::Utf8)]
    #[parent(Info)]
    MuxingApp,

    #[id(0x5741)]
    #[data_type(TagDataType::Utf8)]
    #[parent(Info)]
    WritingApp,

    // Cluster

    #[id(0x1f43b675)]
    #[data_type(TagDataType::Master)]
    #[parent(Segment)]
    Cluster,

    #[id(0xe7)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(Cluster)]
    Timecode,

    #[id(0x5854)]
    #[data_type(TagDataType::Master)]
    #[parent(Cluster)]
    SilentTracks,

    #[id(0x58d7)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(SilentTracks)]
    SilentTrackNumber,

    #[id(0xa7)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(Cluster)]
    Position,

    #[id(0xab)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(Cluster)]
    PrevSize,

    #[id(0xa3)]
    #[data_type(TagDataType::Binary)]
    #[parent(Cluster)]
    SimpleBlock,

    #[id(0xa0)]
    #[data_type(TagDataType::Master)]
    #[parent(Cluster)]
    BlockGroup,

    #[id(0xa1)]
    #[data_type(TagDataType::Binary)]
    #[parent(BlockGroup)]
    Block,

    #[id(0xa2)]
    #[data_type(TagDataType::Binary)]
    #[parent(BlockGroup)]
    BlockVirtual,

    #[id(0x75a1)]
    #[data_type(TagDataType::Master)]
    #[parent(BlockGroup)]
    BlockAdditions,

    #[id(0xa6)]
    #[data_type(TagDataType::Master)]
    #[parent(BlockAdditions)]
    BlockMore,

    #[id(0xee)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(BlockMore)]
    BlockAddId,

    #[id(0xa5)]
    #[data_type(TagDataType::Binary)]
    #[parent(BlockMore)]
    BlockAdditional,

    #[id(0x9b)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(BlockGroup)]
    BlockDuration,

    #[id(0xfa)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(BlockGroup)]
    ReferencePriority,

    #[id(0xfb)]
    #[data_type(TagDataType::Integer)]
    #[parent(BlockGroup)]
    ReferenceBlock,

    #[id(0xfd)]
    #[data_type(TagDataType::Integer)]
    #[parent(BlockGroup)]
    ReferenceVirtual,

    #[id(0xa4)]
    #[data_type(TagDataType::Binary)]
    #[parent(BlockGroup)]
    CodecState,

    #[id(0x75a2)]
    #[data_type(TagDataType::Integer)]
    #[parent(BlockGroup)]
    DiscardPadding,

    #[id(0x8e)]
    #[data_type(TagDataType::Master)]
    #[parent(BlockGroup)]
    Slices,

    #[id(0xe8)]
    #[data_type(TagDataType::Master)]
    #[parent(Slices)]
    TimeSlice,

    #[id(0xcc)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(TimeSlice)]
    LaceNumber,

    #[id(0xcd)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(TimeSlice)]
    FrameNumber,

    #[id(0xcb)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(TimeSlice)]
    BlockAdditionId,

    #[id(0xce)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(TimeSlice)]
    Delay,

    #[id(0xcf)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(TimeSlice)]
    SliceDuration,

    #[id(0xc8)]
    #[data_type(TagDataType::Master)]
    #[parent(BlockGroup)]
    ReferenceFrame,

    #[id(0xc9)]
    #[data_type(TagDataType::UnsignedInt)]
    #[parent(ReferenceFrame)]
    ReferenceOffset,

    #[id(0xca)]
    #[data_type(TagDataType::UnsignedInt)]
    ReferenceTimeCode,

    #[id(0xaf)]
    #[data_type(TagDataType::Binary)]
    EncryptedBlock,


    #[id(0x56bb)]
    #[data_type(TagDataType::UnsignedInt)]
    SeekPreRoll,

    #[id(0x80)]
    #[data_type(TagDataType::Master)]
    ChapterDisplay,
    #[id(0x83)]
    #[data_type(TagDataType::UnsignedInt)]
    TrackType,
    #[id(0x85)]
    #[data_type(TagDataType::Utf8)]
    ChapString,
    #[id(0x86)]
    #[data_type(TagDataType::Utf8)]
    CodecId,
    #[id(0x88)]
    #[data_type(TagDataType::UnsignedInt)]
    FlagDefault,
    #[id(0x89)]
    #[data_type(TagDataType::UnsignedInt)]
    ChapterTrackNumber,
    #[id(0x91)]
    #[data_type(TagDataType::UnsignedInt)]
    ChapterTimeStart,
    #[id(0x92)]
    #[data_type(TagDataType::UnsignedInt)]
    ChapterTimeEnd,
    #[id(0x96)]
    #[data_type(TagDataType::UnsignedInt)]
    CueRefTime,
    #[id(0x97)]
    #[data_type(TagDataType::UnsignedInt)]
    CueRefCluster,
    #[id(0x98)]
    #[data_type(TagDataType::UnsignedInt)]
    ChapterFlagHidden,
    #[id(0x4254)]
    #[data_type(TagDataType::UnsignedInt)]
    ContentCompAlgo,
    #[id(0x4255)]
    #[data_type(TagDataType::Binary)]
    ContentCompSettings,
    #[id(0x4484)]
    #[data_type(TagDataType::UnsignedInt)]
    TagDefault,
    #[id(0x4485)]
    #[data_type(TagDataType::Binary)]
    TagBinary,
    #[id(0x4487)]
    #[data_type(TagDataType::Utf8)]
    TagString,
    #[id(0x4598)]
    #[data_type(TagDataType::UnsignedInt)]
    ChapterFlagEnabled,
    #[id(0x4660)]
    #[data_type(TagDataType::Utf8)]
    FileMimeType,
    #[id(0x4661)]
    #[data_type(TagDataType::UnsignedInt)]
    FileUsedStartTime,
    #[id(0x4662)]
    #[data_type(TagDataType::UnsignedInt)]
    FileUsedEndTime,
    #[id(0x4675)]
    #[data_type(TagDataType::Binary)]
    FileReferral,
    #[id(0x5031)]
    #[data_type(TagDataType::UnsignedInt)]
    ContentEncodingOrder,
    #[id(0x5032)]
    #[data_type(TagDataType::UnsignedInt)]
    ContentEncodingScope,
    #[id(0x5033)]
    #[data_type(TagDataType::UnsignedInt)]
    ContentEncodingType,
    #[id(0x5034)]
    #[data_type(TagDataType::Master)]
    ContentCompression,
    #[id(0x5035)]
    #[data_type(TagDataType::Master)]
    ContentEncryption,
    #[id(0x5378)]
    #[data_type(TagDataType::UnsignedInt)]
    CueBlockNumber,
    #[id(0x5654)]
    #[data_type(TagDataType::Utf8)]
    ChapterStringUid,
    #[id(0x6240)]
    #[data_type(TagDataType::Master)]
    ContentEncoding,
    #[id(0x6264)]
    #[data_type(TagDataType::UnsignedInt)]
    BitDepth,
    #[id(0x6532)]
    #[data_type(TagDataType::Binary)]
    SignedElement,
    #[id(0x6624)]
    #[data_type(TagDataType::Master)]
    TrackTranslate,
    #[id(0x6911)]
    #[data_type(TagDataType::Master)]
    ChapProcessCommand,
    #[id(0x6922)]
    #[data_type(TagDataType::UnsignedInt)]
    ChapProcessTime,
    #[id(0x6933)]
    #[data_type(TagDataType::Binary)]
    ChapProcessData,
    #[id(0x6944)]
    #[data_type(TagDataType::Master)]
    ChapProcess,
    #[id(0x6955)]
    #[data_type(TagDataType::UnsignedInt)]
    ChapProcessCodecId,
    #[id(0x7373)]
    #[data_type(TagDataType::Master)]
    Tag,
    #[id(0x7446)]
    #[data_type(TagDataType::UnsignedInt)]
    AttachmentLink,
    #[id(0x258688)]
    #[data_type(TagDataType::Utf8)]
    CodecName,
    #[id(0x447a)]
    #[data_type(TagDataType::Utf8)]
    TagLanguage,
    #[id(0x45a3)]
    #[data_type(TagDataType::Utf8)]
    TagName,
    #[id(0x67c8)]
    #[data_type(TagDataType::Master)]
    SimpleTag,
    #[id(0x63c6)]
    #[data_type(TagDataType::UnsignedInt)]
    TagAttachmentUid,
    #[id(0x63c4)]
    #[data_type(TagDataType::UnsignedInt)]
    TagChapterUid,
    #[id(0x63c9)]
    #[data_type(TagDataType::UnsignedInt)]
    TagEditionUid,
    #[id(0x63c5)]
    #[data_type(TagDataType::UnsignedInt)]
    TagTrackUid,
    #[id(0x63ca)]
    #[data_type(TagDataType::Utf8)]
    TargetType,
    #[id(0x68ca)]
    #[data_type(TagDataType::UnsignedInt)]
    TargetTypeValue,
    #[id(0x63c0)]
    #[data_type(TagDataType::Master)]
    Targets,
    #[id(0x1254c367)]
    #[data_type(TagDataType::Master)]
    Tags,
    #[id(0x450d)]
    #[data_type(TagDataType::Binary)]
    ChapProcessPrivate,
    #[id(0x437e)]
    #[data_type(TagDataType::Utf8)]
    ChapCountry,
    #[id(0x437c)]
    #[data_type(TagDataType::Utf8)]
    ChapLanguage,
    #[id(0x8f)]
    #[data_type(TagDataType::Master)]
    ChapterTrack,
    #[id(0x63c3)]
    #[data_type(TagDataType::UnsignedInt)]
    ChapterPhysicalEquiv,
    #[id(0x6ebc)]
    #[data_type(TagDataType::UnsignedInt)]
    ChapterSegmentEditionUid,
    #[id(0x73c4)]
    #[data_type(TagDataType::UnsignedInt)]
    ChapterUid,
    #[id(0xb6)]
    #[data_type(TagDataType::Master)]
    ChapterAtom,
    #[id(0x45dd)]
    #[data_type(TagDataType::UnsignedInt)]
    EditionFlagOrdered,
    #[id(0x45db)]
    #[data_type(TagDataType::UnsignedInt)]
    EditionFlagDefault,
    #[id(0x45bd)]
    #[data_type(TagDataType::UnsignedInt)]
    EditionFlagHidden,
    #[id(0x45bc)]
    #[data_type(TagDataType::UnsignedInt)]
    EditionUid,
    #[id(0x45b9)]
    #[data_type(TagDataType::Master)]
    EditionEntry,
    #[id(0x1043a770)]
    #[data_type(TagDataType::Master)]
    Chapters,
    #[id(0x46ae)]
    #[data_type(TagDataType::UnsignedInt)]
    FileUid,
    #[id(0x465c)]
    #[data_type(TagDataType::Binary)]
    FileData,
    #[id(0x466e)]
    #[data_type(TagDataType::Utf8)]
    FileName,
    #[id(0x467e)]
    #[data_type(TagDataType::Utf8)]
    FileDescription,
    #[id(0x61a7)]
    #[data_type(TagDataType::Master)]
    AttachedFile,
    #[id(0x1941a469)]
    #[data_type(TagDataType::Master)]
    Attachments,
    #[id(0xeb)]
    #[data_type(TagDataType::UnsignedInt)]
    CueRefCodecState,
    #[id(0x535f)]
    #[data_type(TagDataType::UnsignedInt)]
    CueRefNumber,
    #[id(0xdb)]
    #[data_type(TagDataType::Master)]
    CueReference,
    #[id(0xea)]
    #[data_type(TagDataType::UnsignedInt)]
    CueCodecState,
    #[id(0xb2)]
    #[data_type(TagDataType::UnsignedInt)]
    CueDuration,
    #[id(0xf0)]
    #[data_type(TagDataType::UnsignedInt)]
    CueRelativePosition,
    #[id(0xf1)]
    #[data_type(TagDataType::UnsignedInt)]
    CueClusterPosition,
    #[id(0xf7)]
    #[data_type(TagDataType::UnsignedInt)]
    CueTrack,
    #[id(0xb7)]
    #[data_type(TagDataType::Master)]
    CueTrackPositions,
    #[id(0xb3)]
    #[data_type(TagDataType::UnsignedInt)]
    CueTime,
    #[id(0xbb)]
    #[data_type(TagDataType::Master)]
    CuePoint,
    #[id(0x1c53bb6b)]
    #[data_type(TagDataType::Master)]
    Cues,
    #[id(0x47e8)]
    #[data_type(TagDataType::UnsignedInt)]
    AesSettingsCipherMode,
    #[id(0x47e7)]
    #[data_type(TagDataType::Master)]
    ContentEncAesSettings,
    #[id(0x47e6)]
    #[data_type(TagDataType::UnsignedInt)]
    ContentSigHashAlgo,
    #[id(0x47e5)]
    #[data_type(TagDataType::UnsignedInt)]
    ContentSigAlgo,
    #[id(0x47e4)]
    #[data_type(TagDataType::Binary)]
    ContentSigKeyId,
    #[id(0x47e3)]
    #[data_type(TagDataType::Binary)]
    ContentSignature,
    #[id(0x47e2)]
    #[data_type(TagDataType::Binary)]
    ContentEncKeyId,
    #[id(0x47e1)]
    #[data_type(TagDataType::UnsignedInt)]
    ContentEncAlgo,
    #[id(0x6d80)]
    #[data_type(TagDataType::Master)]
    ContentEncodings,
    #[id(0xc4)]
    #[data_type(TagDataType::Binary)]
    TrickMasterTrackSegmentUid,
    #[id(0xc7)]
    #[data_type(TagDataType::UnsignedInt)]
    TrickMasterTrackUid,
    #[id(0xc6)]
    #[data_type(TagDataType::UnsignedInt)]
    TrickTrackFlag,
    #[id(0xc1)]
    #[data_type(TagDataType::Binary)]
    TrickTrackSegmentUid,
    #[id(0xc0)]
    #[data_type(TagDataType::UnsignedInt)]
    TrickTrackUid,
    #[id(0xed)]
    #[data_type(TagDataType::UnsignedInt)]
    TrackJoinUid,
    #[id(0xe9)]
    #[data_type(TagDataType::Master)]
    TrackJoinBlocks,
    #[id(0xe6)]
    #[data_type(TagDataType::UnsignedInt)]
    TrackPlaneType,
    #[id(0xe5)]
    #[data_type(TagDataType::UnsignedInt)]
    TrackPlaneUid,
    #[id(0xe4)]
    #[data_type(TagDataType::Master)]
    TrackPlane,
    #[id(0xe3)]
    #[data_type(TagDataType::Master)]
    TrackCombinePlanes,
    #[id(0xe2)]
    #[data_type(TagDataType::Master)]
    TrackOperation,
    #[id(0x7d7b)]
    #[data_type(TagDataType::Binary)]
    ChannelPositions,
    #[id(0x9f)]
    #[data_type(TagDataType::UnsignedInt)]
    Channels,
    #[id(0x78b5)]
    #[data_type(TagDataType::Float)]
    OutputSamplingFrequency,
    #[id(0xb5)]
    #[data_type(TagDataType::Float)]
    SamplingFrequency,
    #[id(0xe1)]
    #[data_type(TagDataType::Master)]
    Audio,
    #[id(0x2383e3)]
    #[data_type(TagDataType::Float)]
    FrameRate,
    #[id(0x2fb523)]
    #[data_type(TagDataType::Float)]
    GammaValue,
    #[id(0x2eb524)]
    #[data_type(TagDataType::Binary)]
    ColourSpace,
    #[id(0x54b3)]
    #[data_type(TagDataType::UnsignedInt)]
    AspectRatioType,
    #[id(0x54b2)]
    #[data_type(TagDataType::UnsignedInt)]
    DisplayUnit,
    #[id(0x54ba)]
    #[data_type(TagDataType::UnsignedInt)]
    DisplayHeight,
    #[id(0x54b0)]
    #[data_type(TagDataType::UnsignedInt)]
    DisplayWidth,
    #[id(0x54dd)]
    #[data_type(TagDataType::UnsignedInt)]
    PixelCropRight,
    #[id(0x54cc)]
    #[data_type(TagDataType::UnsignedInt)]
    PixelCropLeft,
    #[id(0x54bb)]
    #[data_type(TagDataType::UnsignedInt)]
    PixelCropTop,
    #[id(0x54aa)]
    #[data_type(TagDataType::UnsignedInt)]
    PixelCropBottom,
    #[id(0xba)]
    #[data_type(TagDataType::UnsignedInt)]
    PixelHeight,
    #[id(0xb0)]
    #[data_type(TagDataType::UnsignedInt)]
    PixelWidth,
    #[id(0x53b9)]
    #[data_type(TagDataType::UnsignedInt)]
    OldStereoMode,
    #[id(0x53c0)]
    #[data_type(TagDataType::UnsignedInt)]
    AlphaMode,
    #[id(0x53b8)]
    #[data_type(TagDataType::UnsignedInt)]
    StereoMode,
    #[id(0x9a)]
    #[data_type(TagDataType::UnsignedInt)]
    FlagInterlaced,
    #[id(0xe0)]
    #[data_type(TagDataType::Master)]
    Video,
    #[id(0x66a5)]
    #[data_type(TagDataType::Binary)]
    TrackTranslateTrackId,
    #[id(0x66bf)]
    #[data_type(TagDataType::UnsignedInt)]
    TrackTranslateCodec,
    #[id(0x66fc)]
    #[data_type(TagDataType::UnsignedInt)]
    TrackTranslateEditionUid,
    #[id(0x56aa)]
    #[data_type(TagDataType::UnsignedInt)]
    CodecDelay,
    #[id(0x6fab)]
    #[data_type(TagDataType::UnsignedInt)]
    TrackOverlay,
    #[id(0xaa)]
    #[data_type(TagDataType::UnsignedInt)]
    CodecDecodeAll,
    #[id(0x26b240)]
    #[data_type(TagDataType::Utf8)]
    CodecDownloadUrl,
    #[id(0x3b4040)]
    #[data_type(TagDataType::Utf8)]
    CodecInfoUrl,
    #[id(0x3a9697)]
    #[data_type(TagDataType::Utf8)]
    CodecSettings,
    #[id(0x63a2)]
    #[data_type(TagDataType::Binary)]
    CodecPrivate,
    #[id(0x22b59c)]
    #[data_type(TagDataType::Utf8)]
    Language,
    #[id(0x22b59d)]
    #[data_type(TagDataType::Utf8)]
    LanguageIETF,
    #[id(0x536e)]
    #[data_type(TagDataType::Utf8)]
    Name,
    #[id(0x55ee)]
    #[data_type(TagDataType::UnsignedInt)]
    MaxBlockAdditionId,
    #[id(0x537f)]
    #[data_type(TagDataType::Integer)]
    TrackOffset,
    #[id(0x23314f)]
    #[data_type(TagDataType::Float)]
    TrackTimecodeScale,
    #[id(0x234e7a)]
    #[data_type(TagDataType::UnsignedInt)]
    DefaultDecodedFieldDuration,
    #[id(0x23e383)]
    #[data_type(TagDataType::UnsignedInt)]
    DefaultDuration,
    #[id(0x6df8)]
    #[data_type(TagDataType::UnsignedInt)]
    MaxCache,
    #[id(0x6de7)]
    #[data_type(TagDataType::UnsignedInt)]
    MinCache,
    #[id(0x9c)]
    #[data_type(TagDataType::UnsignedInt)]
    FlagLacing,
    #[id(0x55aa)]
    #[data_type(TagDataType::UnsignedInt)]
    FlagForced,
    #[id(0xb9)]
    #[data_type(TagDataType::UnsignedInt)]
    FlagEnabled,
    #[id(0x73c5)]
    #[data_type(TagDataType::UnsignedInt)]
    TrackUid,
    #[id(0xd7)]
    #[data_type(TagDataType::UnsignedInt)]
    TrackNumber,
    #[id(0xae)]
    #[data_type(TagDataType::Master)]
    TrackEntry,
    #[id(0x1654ae6b)]
    #[data_type(TagDataType::Master)]
    Tracks,
    #[id(0x2ad7b2)]
    #[data_type(TagDataType::UnsignedInt)]
    TimecodeScaleDenominator,
    #[id(0x4dbb)]
    #[data_type(TagDataType::Master)]
    Seek,
    #[id(0x7e7b)]
    #[data_type(TagDataType::Master)]
    SignatureElementList,
    #[id(0x7e5b)]
    #[data_type(TagDataType::Master)]
    SignatureElements,
    #[id(0x7eb5)]
    #[data_type(TagDataType::Binary)]
    Signature,
    #[id(0x7ea5)]
    #[data_type(TagDataType::Binary)]
    SignaturePublicKey,
    #[id(0x7e9a)]
    #[data_type(TagDataType::UnsignedInt)]
    SignatureHash,
    #[id(0x7e8a)]
    #[data_type(TagDataType::UnsignedInt)]
    SignatureAlgo,
    #[id(0x1b538667)]
    #[data_type(TagDataType::Master)]
    SignatureSlot,
}
