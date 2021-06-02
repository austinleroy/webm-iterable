mod block;
mod simple_block;

pub use block::{Block, BlockLacing};
pub use simple_block::SimpleBlock;

use ebml_iterable::specs::{TagSpec, SpecTagType};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MatroskaTag {
    Block,
    SimpleBlock,
    ChapterDisplay,
    ContentCompression,
    ContentEncryption,
    SilentTracks,
    ContentEncoding,
    TrackTranslate,
    ChapProcessCommand,
    ChapterTranslate,
    ChapProcess,
    Tag,
    Segment,
    SimpleTag,
    Targets,
    Tags,
    ChapterTrack,
    ChapterAtom,
    EditionEntry,
    Chapters,
    AttachedFile,
    Attachments,
    CueReference,
    CueTrackPositions,
    CuePoint,
    Cues,
    ContentEncAesSettings,
    ContentEncodings,
    TrackJoinBlocks,
    TrackPlane,
    TrackCombinePlanes,
    TrackOperation,
    Audio,
    Video,
    TrackEntry,
    Tracks,
    ReferenceFrame,
    TimeSlice,
    Slices,
    BlockMore,
    BlockAdditions,
    BlockGroup,
    Cluster,
    Info,
    Seek,
    SeekHead,
    SignatureElementList,
    SignatureElements,
    SignatureSlot,
    Ebml,
    TrackType,
    FlagDefault,
    ChapterTrackNumber,
    ChapterTimeStart,
    ChapterTimeEnd,
    CueRefTime,
    CueRefCluster,
    ChapterFlagHidden,
    ContentCompAlgo,
    DocTypeReadVersion,
    EbmlVersion,
    DocTypeVersion,
    TagDefault,
    ChapterFlagEnabled,
    FileUsedStartTime,
    FileUsedEndTime,
    ContentEncodingOrder,
    ContentEncodingScope,
    ContentEncodingType,
    CueBlockNumber,
    BitDepth,
    ChapProcessTime,
    ChapProcessCodecId,
    AttachmentLink,
    TagAttachmentUid,
    TagChapterUid,
    TagEditionUid,
    TagTrackUid,
    TargetTypeValue,
    ChapterPhysicalEquiv,
    ChapterSegmentEditionUid,
    ChapterUid,
    EditionFlagOrdered,
    EditionFlagDefault,
    EditionFlagHidden,
    EditionUid,
    FileUid,
    CueRefCodecState,
    CueRefNumber,
    CueCodecState,
    CueDuration,
    CueRelativePosition,
    CueClusterPosition,
    CueTrack,
    CueTime,
    AesSettingsCipherMode,
    ContentSigHashAlgo,
    ContentSigAlgo,
    ContentEncAlgo,
    TrickMasterTrackUid,
    TrickTrackFlag,
    TrickTrackUid,
    TrackJoinUid,
    TrackPlaneType,
    TrackPlaneUid,
    Channels,
    AspectRatioType,
    DisplayUnit,
    DisplayHeight,
    DisplayWidth,
    PixelCropRight,
    PixelCropLeft,
    PixelCropTop,
    PixelCropBottom,
    PixelHeight,
    PixelWidth,
    OldStereoMode,
    AlphaMode,
    StereoMode,
    FlagInterlaced,
    TrackTranslateCodec,
    TrackTranslateEditionUid,
    SeekPreRoll,
    CodecDelay,
    TrackOverlay,
    CodecDecodeAll,
    MaxBlockAdditionId,
    DefaultDecodedFieldDuration,
    DefaultDuration,
    MaxCache,
    MinCache,
    FlagLacing,
    FlagForced,
    FlagEnabled,
    TrackUid,
    TrackNumber,
    ReferenceTimeCode,
    ReferenceOffset,
    SliceDuration,
    Delay,
    BlockAdditionId,
    FrameNumber,
    LaceNumber,
    ReferencePriority,
    BlockDuration,
    BlockAddId,
    PrevSize,
    Position,
    SilentTrackNumber,
    Timecode,
    TimecodeScaleDenominator,
    TimecodeScale,
    ChapterTranslateCodec,
    ChapterTranslateEditionUid,
    SeekPosition,
    SignatureHash,
    SignatureAlgo,
    EbmlMaxSizeLength,
    EbmlMaxIdLength,
    EbmlReadVersion,
    TrackOffset,
    DiscardPadding,
    ReferenceVirtual,
    ReferenceBlock,
    CodecId,
    DocType,
    FileMimeType,
    TagLanguage,
    TargetType,
    ChapCountry,
    ChapLanguage,
    CodecDownloadUrl,
    CodecInfoUrl,
    Language,
    ChapString,
    TagString,
    ChapterStringUid,
    WritingApp,
    SegmentFilename,
    CodecName,
    TagName,
    FileName,
    FileDescription,
    CodecSettings,
    Name,
    MuxingApp,
    Title,
    NextFilename,
    PrevFilename,
    ContentCompSettings,
    SegmentFamily,
    TagBinary,
    FileReferral,
    SignedElement,
    ChapProcessData,
    ChapProcessPrivate,
    ChapterSegmentUid,
    FileData,
    ContentSigKeyId,
    ContentSignature,
    ContentEncKeyId,
    TrickMasterTrackSegmentUid,
    TrickTrackSegmentUid,
    ChannelPositions,
    ColourSpace,
    TrackTranslateTrackId,
    CodecPrivate,
    EncryptedBlock,
    CodecState,
    BlockAdditional,
    BlockVirtual,
    ChapterTranslateId,
    NextUid,
    PrevUid,
    SegmentUid,
    SeekId,
    Signature,
    SignaturePublicKey,
    Crc32,
    Void,
    DateUtc,
    Duration,
    OutputSamplingFrequency,
    SamplingFrequency,
    FrameRate,
    GammaValue,
    TrackTimecodeScale,

    Unknown,
}

#[derive(Default)]
pub struct MatroskaSpec { }

impl TagSpec for MatroskaSpec {
    type SpecType = MatroskaTag;

    fn get_tag(&self, id: u64) -> Self::SpecType {
        if id == 0x80 {
            return MatroskaTag::ChapterDisplay;
        }
        if id == 0x83 {
            return MatroskaTag::TrackType;
        }
        if id == 0x85 {
            return MatroskaTag::ChapString;
        }
        if id == 0x86 {
            return MatroskaTag::CodecId;
        }
        if id == 0x88 {
            return MatroskaTag::FlagDefault;
        }
        if id == 0x89 {
            return MatroskaTag::ChapterTrackNumber;
        }
        if id == 0x91 {
            return MatroskaTag::ChapterTimeStart;
        }
        if id == 0x92 {
            return MatroskaTag::ChapterTimeEnd;
        }
        if id == 0x96 {
            return MatroskaTag::CueRefTime;
        }
        if id == 0x97 {
            return MatroskaTag::CueRefCluster;
        }
        if id == 0x98 {
            return MatroskaTag::ChapterFlagHidden;
        }
        if id == 0x4254 {
            return MatroskaTag::ContentCompAlgo;
        }
        if id == 0x4255 {
            return MatroskaTag::ContentCompSettings;
        }
        if id == 0x4282 {
            return MatroskaTag::DocType;
        }
        if id == 0x4285 {
            return MatroskaTag::DocTypeReadVersion;
        }
        if id == 0x4286 {
            return MatroskaTag::EbmlVersion;
        }
        if id == 0x4287 {
            return MatroskaTag::DocTypeVersion;
        }
        if id == 0x4444 {
            return MatroskaTag::SegmentFamily;
        }
        if id == 0x4461 {
            return MatroskaTag::DateUtc;
        }
        if id == 0x4484 {
            return MatroskaTag::TagDefault;
        }
        if id == 0x4485 {
            return MatroskaTag::TagBinary;
        }
        if id == 0x4487 {
            return MatroskaTag::TagString;
        }
        if id == 0x4489 {
            return MatroskaTag::Duration;
        }
        if id == 0x4598 {
            return MatroskaTag::ChapterFlagEnabled;
        }
        if id == 0x4660 {
            return MatroskaTag::FileMimeType;
        }
        if id == 0x4661 {
            return MatroskaTag::FileUsedStartTime;
        }
        if id == 0x4662 {
            return MatroskaTag::FileUsedEndTime;
        }
        if id == 0x4675 {
            return MatroskaTag::FileReferral;
        }
        if id == 0x5031 {
            return MatroskaTag::ContentEncodingOrder;
        }
        if id == 0x5032 {
            return MatroskaTag::ContentEncodingScope;
        }
        if id == 0x5033 {
            return MatroskaTag::ContentEncodingType;
        }
        if id == 0x5034 {
            return MatroskaTag::ContentCompression;
        }
        if id == 0x5035 {
            return MatroskaTag::ContentEncryption;
        }
        if id == 0x5378 {
            return MatroskaTag::CueBlockNumber;
        }
        if id == 0x5654 {
            return MatroskaTag::ChapterStringUid;
        }
        if id == 0x5741 {
            return MatroskaTag::WritingApp;
        }
        if id == 0x5854 {
            return MatroskaTag::SilentTracks;
        }
        if id == 0x6240 {
            return MatroskaTag::ContentEncoding;
        }
        if id == 0x6264 {
            return MatroskaTag::BitDepth;
        }
        if id == 0x6532 {
            return MatroskaTag::SignedElement;
        }
        if id == 0x6624 {
            return MatroskaTag::TrackTranslate;
        }
        if id == 0x6911 {
            return MatroskaTag::ChapProcessCommand;
        }
        if id == 0x6922 {
            return MatroskaTag::ChapProcessTime;
        }
        if id == 0x6924 {
            return MatroskaTag::ChapterTranslate;
        }
        if id == 0x6933 {
            return MatroskaTag::ChapProcessData;
        }
        if id == 0x6944 {
            return MatroskaTag::ChapProcess;
        }
        if id == 0x6955 {
            return MatroskaTag::ChapProcessCodecId;
        }
        if id == 0x7373 {
            return MatroskaTag::Tag;
        }
        if id == 0x7384 {
            return MatroskaTag::SegmentFilename;
        }
        if id == 0x7446 {
            return MatroskaTag::AttachmentLink;
        }
        if id == 0x258688 {
            return MatroskaTag::CodecName;
        }
        if id == 0x18538067 {
            return MatroskaTag::Segment;
        }
        if id == 0x447a {
            return MatroskaTag::TagLanguage;
        }
        if id == 0x45a3 {
            return MatroskaTag::TagName;
        }
        if id == 0x67c8 {
            return MatroskaTag::SimpleTag;
        }
        if id == 0x63c6 {
            return MatroskaTag::TagAttachmentUid;
        }
        if id == 0x63c4 {
            return MatroskaTag::TagChapterUid;
        }
        if id == 0x63c9 {
            return MatroskaTag::TagEditionUid;
        }
        if id == 0x63c5 {
            return MatroskaTag::TagTrackUid;
        }
        if id == 0x63ca {
            return MatroskaTag::TargetType;
        }
        if id == 0x68ca {
            return MatroskaTag::TargetTypeValue;
        }
        if id == 0x63c0 {
            return MatroskaTag::Targets;
        }
        if id == 0x1254c367 {
            return MatroskaTag::Tags;
        }
        if id == 0x450d {
            return MatroskaTag::ChapProcessPrivate;
        }
        if id == 0x437e {
            return MatroskaTag::ChapCountry;
        }
        if id == 0x437c {
            return MatroskaTag::ChapLanguage;
        }
        if id == 0x8f {
            return MatroskaTag::ChapterTrack;
        }
        if id == 0x63c3 {
            return MatroskaTag::ChapterPhysicalEquiv;
        }
        if id == 0x6ebc {
            return MatroskaTag::ChapterSegmentEditionUid;
        }
        if id == 0x6e67 {
            return MatroskaTag::ChapterSegmentUid;
        }
        if id == 0x73c4 {
            return MatroskaTag::ChapterUid;
        }
        if id == 0xb6 {
            return MatroskaTag::ChapterAtom;
        }
        if id == 0x45dd {
            return MatroskaTag::EditionFlagOrdered;
        }
        if id == 0x45db {
            return MatroskaTag::EditionFlagDefault;
        }
        if id == 0x45bd {
            return MatroskaTag::EditionFlagHidden;
        }
        if id == 0x45bc {
            return MatroskaTag::EditionUid;
        }
        if id == 0x45b9 {
            return MatroskaTag::EditionEntry;
        }
        if id == 0x1043a770 {
            return MatroskaTag::Chapters;
        }
        if id == 0x46ae {
            return MatroskaTag::FileUid;
        }
        if id == 0x465c {
            return MatroskaTag::FileData;
        }
        if id == 0x466e {
            return MatroskaTag::FileName;
        }
        if id == 0x467e {
            return MatroskaTag::FileDescription;
        }
        if id == 0x61a7 {
            return MatroskaTag::AttachedFile;
        }
        if id == 0x1941a469 {
            return MatroskaTag::Attachments;
        }
        if id == 0xeb {
            return MatroskaTag::CueRefCodecState;
        }
        if id == 0x535f {
            return MatroskaTag::CueRefNumber;
        }
        if id == 0xdb {
            return MatroskaTag::CueReference;
        }
        if id == 0xea {
            return MatroskaTag::CueCodecState;
        }
        if id == 0xb2 {
            return MatroskaTag::CueDuration;
        }
        if id == 0xf0 {
            return MatroskaTag::CueRelativePosition;
        }
        if id == 0xf1 {
            return MatroskaTag::CueClusterPosition;
        }
        if id == 0xf7 {
            return MatroskaTag::CueTrack;
        }
        if id == 0xb7 {
            return MatroskaTag::CueTrackPositions;
        }
        if id == 0xb3 {
            return MatroskaTag::CueTime;
        }
        if id == 0xbb {
            return MatroskaTag::CuePoint;
        }
        if id == 0x1c53bb6b {
            return MatroskaTag::Cues;
        }
        if id == 0x47e8 {
            return MatroskaTag::AesSettingsCipherMode;
        }
        if id == 0x47e7 {
            return MatroskaTag::ContentEncAesSettings;
        }
        if id == 0x47e6 {
            return MatroskaTag::ContentSigHashAlgo;
        }
        if id == 0x47e5 {
            return MatroskaTag::ContentSigAlgo;
        }
        if id == 0x47e4 {
            return MatroskaTag::ContentSigKeyId;
        }
        if id == 0x47e3 {
            return MatroskaTag::ContentSignature;
        }
        if id == 0x47e2 {
            return MatroskaTag::ContentEncKeyId;
        }
        if id == 0x47e1 {
            return MatroskaTag::ContentEncAlgo;
        }
        if id == 0x6d80 {
            return MatroskaTag::ContentEncodings;
        }
        if id == 0xc4 {
            return MatroskaTag::TrickMasterTrackSegmentUid;
        }
        if id == 0xc7 {
            return MatroskaTag::TrickMasterTrackUid;
        }
        if id == 0xc6 {
            return MatroskaTag::TrickTrackFlag;
        }
        if id == 0xc1 {
            return MatroskaTag::TrickTrackSegmentUid;
        }
        if id == 0xc0 {
            return MatroskaTag::TrickTrackUid;
        }
        if id == 0xed {
            return MatroskaTag::TrackJoinUid;
        }
        if id == 0xe9 {
            return MatroskaTag::TrackJoinBlocks;
        }
        if id == 0xe6 {
            return MatroskaTag::TrackPlaneType;
        }
        if id == 0xe5 {
            return MatroskaTag::TrackPlaneUid;
        }
        if id == 0xe4 {
            return MatroskaTag::TrackPlane;
        }
        if id == 0xe3 {
            return MatroskaTag::TrackCombinePlanes;
        }
        if id == 0xe2 {
            return MatroskaTag::TrackOperation;
        }
        if id == 0x7d7b {
            return MatroskaTag::ChannelPositions;
        }
        if id == 0x9f {
            return MatroskaTag::Channels;
        }
        if id == 0x78b5 {
            return MatroskaTag::OutputSamplingFrequency;
        }
        if id == 0xb5 {
            return MatroskaTag::SamplingFrequency;
        }
        if id == 0xe1 {
            return MatroskaTag::Audio;
        }
        if id == 0x2383e3 {
            return MatroskaTag::FrameRate;
        }
        if id == 0x2fb523 {
            return MatroskaTag::GammaValue;
        }
        if id == 0x2eb524 {
            return MatroskaTag::ColourSpace;
        }
        if id == 0x54b3 {
            return MatroskaTag::AspectRatioType;
        }
        if id == 0x54b2 {
            return MatroskaTag::DisplayUnit;
        }
        if id == 0x54ba {
            return MatroskaTag::DisplayHeight;
        }
        if id == 0x54b0 {
            return MatroskaTag::DisplayWidth;
        }
        if id == 0x54dd {
            return MatroskaTag::PixelCropRight;
        }
        if id == 0x54cc {
            return MatroskaTag::PixelCropLeft;
        }
        if id == 0x54bb {
            return MatroskaTag::PixelCropTop;
        }
        if id == 0x54aa {
            return MatroskaTag::PixelCropBottom;
        }
        if id == 0xba {
            return MatroskaTag::PixelHeight;
        }
        if id == 0xb0 {
            return MatroskaTag::PixelWidth;
        }
        if id == 0x53b9 {
            return MatroskaTag::OldStereoMode;
        }
        if id == 0x53c0 {
            return MatroskaTag::AlphaMode;
        }
        if id == 0x53b8 {
            return MatroskaTag::StereoMode;
        }
        if id == 0x9a {
            return MatroskaTag::FlagInterlaced;
        }
        if id == 0xe0 {
            return MatroskaTag::Video;
        }
        if id == 0x66a5 {
            return MatroskaTag::TrackTranslateTrackId;
        }
        if id == 0x66bf {
            return MatroskaTag::TrackTranslateCodec;
        }
        if id == 0x66fc {
            return MatroskaTag::TrackTranslateEditionUid;
        }
        if id == 0x56bb {
            return MatroskaTag::SeekPreRoll;
        }
        if id == 0x56aa {
            return MatroskaTag::CodecDelay;
        }
        if id == 0x6fab {
            return MatroskaTag::TrackOverlay;
        }
        if id == 0xaa {
            return MatroskaTag::CodecDecodeAll;
        }
        if id == 0x26b240 {
            return MatroskaTag::CodecDownloadUrl;
        }
        if id == 0x3b4040 {
            return MatroskaTag::CodecInfoUrl;
        }
        if id == 0x3a9697 {
            return MatroskaTag::CodecSettings;
        }
        if id == 0x63a2 {
            return MatroskaTag::CodecPrivate;
        }
        if id == 0x22b59c {
            return MatroskaTag::Language;
        }
        if id == 0x536e {
            return MatroskaTag::Name;
        }
        if id == 0x55ee {
            return MatroskaTag::MaxBlockAdditionId;
        }
        if id == 0x537f {
            return MatroskaTag::TrackOffset;
        }
        if id == 0x23314f {
            return MatroskaTag::TrackTimecodeScale;
        }
        if id == 0x234e7a {
            return MatroskaTag::DefaultDecodedFieldDuration;
        }
        if id == 0x23e383 {
            return MatroskaTag::DefaultDuration;
        }
        if id == 0x6df8 {
            return MatroskaTag::MaxCache;
        }
        if id == 0x6de7 {
            return MatroskaTag::MinCache;
        }
        if id == 0x9c {
            return MatroskaTag::FlagLacing;
        }
        if id == 0x55aa {
            return MatroskaTag::FlagForced;
        }
        if id == 0xb9 {
            return MatroskaTag::FlagEnabled;
        }
        if id == 0x73c5 {
            return MatroskaTag::TrackUid;
        }
        if id == 0xd7 {
            return MatroskaTag::TrackNumber;
        }
        if id == 0xae {
            return MatroskaTag::TrackEntry;
        }
        if id == 0x1654ae6b {
            return MatroskaTag::Tracks;
        }
        if id == 0xaf {
            return MatroskaTag::EncryptedBlock;
        }
        if id == 0xca {
            return MatroskaTag::ReferenceTimeCode;
        }
        if id == 0xc9 {
            return MatroskaTag::ReferenceOffset;
        }
        if id == 0xc8 {
            return MatroskaTag::ReferenceFrame;
        }
        if id == 0xcf {
            return MatroskaTag::SliceDuration;
        }
        if id == 0xce {
            return MatroskaTag::Delay;
        }
        if id == 0xcb {
            return MatroskaTag::BlockAdditionId;
        }
        if id == 0xcd {
            return MatroskaTag::FrameNumber;
        }
        if id == 0xcc {
            return MatroskaTag::LaceNumber;
        }
        if id == 0xe8 {
            return MatroskaTag::TimeSlice;
        }
        if id == 0x8e {
            return MatroskaTag::Slices;
        }
        if id == 0x75a2 {
            return MatroskaTag::DiscardPadding;
        }
        if id == 0xa4 {
            return MatroskaTag::CodecState;
        }
        if id == 0xfd {
            return MatroskaTag::ReferenceVirtual;
        }
        if id == 0xfb {
            return MatroskaTag::ReferenceBlock;
        }
        if id == 0xfa {
            return MatroskaTag::ReferencePriority;
        }
        if id == 0x9b {
            return MatroskaTag::BlockDuration;
        }
        if id == 0xa5 {
            return MatroskaTag::BlockAdditional;
        }
        if id == 0xee {
            return MatroskaTag::BlockAddId;
        }
        if id == 0xa6 {
            return MatroskaTag::BlockMore;
        }
        if id == 0x75a1 {
            return MatroskaTag::BlockAdditions;
        }
        if id == 0xa2 {
            return MatroskaTag::BlockVirtual;
        }
        if id == 0xa1 {
            return MatroskaTag::Block;
        }
        if id == 0xa0 {
            return MatroskaTag::BlockGroup;
        }
        if id == 0xa3 {
            return MatroskaTag::SimpleBlock;
        }
        if id == 0xab {
            return MatroskaTag::PrevSize;
        }
        if id == 0xa7 {
            return MatroskaTag::Position;
        }
        if id == 0x58d7 {
            return MatroskaTag::SilentTrackNumber;
        }
        if id == 0xe7 {
            return MatroskaTag::Timecode;
        }
        if id == 0x1f43b675 {
            return MatroskaTag::Cluster;
        }
        if id == 0x4d80 {
            return MatroskaTag::MuxingApp;
        }
        if id == 0x7ba9 {
            return MatroskaTag::Title;
        }
        if id == 0x2ad7b2 {
            return MatroskaTag::TimecodeScaleDenominator;
        }
        if id == 0x2ad7b1 {
            return MatroskaTag::TimecodeScale;
        }
        if id == 0x69a5 {
            return MatroskaTag::ChapterTranslateId;
        }
        if id == 0x69bf {
            return MatroskaTag::ChapterTranslateCodec;
        }
        if id == 0x69fc {
            return MatroskaTag::ChapterTranslateEditionUid;
        }
        if id == 0x3e83bb {
            return MatroskaTag::NextFilename;
        }
        if id == 0x3eb923 {
            return MatroskaTag::NextUid;
        }
        if id == 0x3c83ab {
            return MatroskaTag::PrevFilename;
        }
        if id == 0x3cb923 {
            return MatroskaTag::PrevUid;
        }
        if id == 0x73a4 {
            return MatroskaTag::SegmentUid;
        }
        if id == 0x1549a966 {
            return MatroskaTag::Info;
        }
        if id == 0x53ac {
            return MatroskaTag::SeekPosition;
        }
        if id == 0x53ab {
            return MatroskaTag::SeekId;
        }
        if id == 0x4dbb {
            return MatroskaTag::Seek;
        }
        if id == 0x114d9b74 {
            return MatroskaTag::SeekHead;
        }
        if id == 0x7e7b {
            return MatroskaTag::SignatureElementList;
        }
        if id == 0x7e5b {
            return MatroskaTag::SignatureElements;
        }
        if id == 0x7eb5 {
            return MatroskaTag::Signature;
        }
        if id == 0x7ea5 {
            return MatroskaTag::SignaturePublicKey;
        }
        if id == 0x7e9a {
            return MatroskaTag::SignatureHash;
        }
        if id == 0x7e8a {
            return MatroskaTag::SignatureAlgo;
        }
        if id == 0x1b538667 {
            return MatroskaTag::SignatureSlot;
        }
        if id == 0xbf {
            return MatroskaTag::Crc32;
        }
        if id == 0xec {
            return MatroskaTag::Void;
        }
        if id == 0x42f3 {
            return MatroskaTag::EbmlMaxSizeLength;
        }
        if id == 0x42f2 {
            return MatroskaTag::EbmlMaxIdLength;
        }
        if id == 0x42f7 {
            return MatroskaTag::EbmlReadVersion;
        }
        if id == 0x1a45dfa3 {
            return MatroskaTag::Ebml;
        }

        MatroskaTag::Unknown
    }

    fn get_tag_type(&self, tag: &Self::SpecType) -> SpecTagType {
        match tag {
            MatroskaTag::Block => SpecTagType::Binary,
            MatroskaTag::SimpleBlock => SpecTagType::Binary,

            MatroskaTag::ChapterDisplay => SpecTagType::Master,
            MatroskaTag::ContentCompression => SpecTagType::Master,
            MatroskaTag::ContentEncryption => SpecTagType::Master,
            MatroskaTag::SilentTracks => SpecTagType::Master,
            MatroskaTag::ContentEncoding => SpecTagType::Master,
            MatroskaTag::TrackTranslate => SpecTagType::Master,
            MatroskaTag::ChapProcessCommand => SpecTagType::Master,
            MatroskaTag::ChapterTranslate => SpecTagType::Master,
            MatroskaTag::ChapProcess => SpecTagType::Master,
            MatroskaTag::Tag => SpecTagType::Master,
            MatroskaTag::Segment => SpecTagType::Master,
            MatroskaTag::SimpleTag => SpecTagType::Master,
            MatroskaTag::Targets => SpecTagType::Master,
            MatroskaTag::Tags => SpecTagType::Master,
            MatroskaTag::ChapterTrack => SpecTagType::Master,
            MatroskaTag::ChapterAtom => SpecTagType::Master,
            MatroskaTag::EditionEntry => SpecTagType::Master,
            MatroskaTag::Chapters => SpecTagType::Master,
            MatroskaTag::AttachedFile => SpecTagType::Master,
            MatroskaTag::Attachments => SpecTagType::Master,
            MatroskaTag::CueReference => SpecTagType::Master,
            MatroskaTag::CueTrackPositions => SpecTagType::Master,
            MatroskaTag::CuePoint => SpecTagType::Master,
            MatroskaTag::Cues => SpecTagType::Master,
            MatroskaTag::ContentEncAesSettings => SpecTagType::Master,
            MatroskaTag::ContentEncodings => SpecTagType::Master,
            MatroskaTag::TrackJoinBlocks => SpecTagType::Master,
            MatroskaTag::TrackPlane => SpecTagType::Master,
            MatroskaTag::TrackCombinePlanes => SpecTagType::Master,
            MatroskaTag::TrackOperation => SpecTagType::Master,
            MatroskaTag::Audio => SpecTagType::Master,
            MatroskaTag::Video => SpecTagType::Master,
            MatroskaTag::TrackEntry => SpecTagType::Master,
            MatroskaTag::Tracks => SpecTagType::Master,
            MatroskaTag::ReferenceFrame => SpecTagType::Master,
            MatroskaTag::TimeSlice => SpecTagType::Master,
            MatroskaTag::Slices => SpecTagType::Master,
            MatroskaTag::BlockMore => SpecTagType::Master,
            MatroskaTag::BlockAdditions => SpecTagType::Master,
            MatroskaTag::BlockGroup => SpecTagType::Master,
            MatroskaTag::Cluster => SpecTagType::Master,
            MatroskaTag::Info => SpecTagType::Master,
            MatroskaTag::Seek => SpecTagType::Master,
            MatroskaTag::SeekHead => SpecTagType::Master,
            MatroskaTag::SignatureElementList => SpecTagType::Master,
            MatroskaTag::SignatureElements => SpecTagType::Master,
            MatroskaTag::SignatureSlot => SpecTagType::Master,
            MatroskaTag::Ebml => SpecTagType::Master,
            MatroskaTag::TrackType => SpecTagType::UnsignedInt,
            MatroskaTag::FlagDefault => SpecTagType::UnsignedInt,
            MatroskaTag::ChapterTrackNumber => SpecTagType::UnsignedInt,
            MatroskaTag::ChapterTimeStart => SpecTagType::UnsignedInt,
            MatroskaTag::ChapterTimeEnd => SpecTagType::UnsignedInt,
            MatroskaTag::CueRefTime => SpecTagType::UnsignedInt,
            MatroskaTag::CueRefCluster => SpecTagType::UnsignedInt,
            MatroskaTag::ChapterFlagHidden => SpecTagType::UnsignedInt,
            MatroskaTag::ContentCompAlgo => SpecTagType::UnsignedInt,
            MatroskaTag::DocTypeReadVersion => SpecTagType::UnsignedInt,
            MatroskaTag::EbmlVersion => SpecTagType::UnsignedInt,
            MatroskaTag::DocTypeVersion => SpecTagType::UnsignedInt,
            MatroskaTag::TagDefault => SpecTagType::UnsignedInt,
            MatroskaTag::ChapterFlagEnabled => SpecTagType::UnsignedInt,
            MatroskaTag::FileUsedStartTime => SpecTagType::UnsignedInt,
            MatroskaTag::FileUsedEndTime => SpecTagType::UnsignedInt,
            MatroskaTag::ContentEncodingOrder => SpecTagType::UnsignedInt,
            MatroskaTag::ContentEncodingScope => SpecTagType::UnsignedInt,
            MatroskaTag::ContentEncodingType => SpecTagType::UnsignedInt,
            MatroskaTag::CueBlockNumber => SpecTagType::UnsignedInt,
            MatroskaTag::BitDepth => SpecTagType::UnsignedInt,
            MatroskaTag::ChapProcessTime => SpecTagType::UnsignedInt,
            MatroskaTag::ChapProcessCodecId => SpecTagType::UnsignedInt,
            MatroskaTag::AttachmentLink => SpecTagType::UnsignedInt,
            MatroskaTag::TagAttachmentUid => SpecTagType::UnsignedInt,
            MatroskaTag::TagChapterUid => SpecTagType::UnsignedInt,
            MatroskaTag::TagEditionUid => SpecTagType::UnsignedInt,
            MatroskaTag::TagTrackUid => SpecTagType::UnsignedInt,
            MatroskaTag::TargetTypeValue => SpecTagType::UnsignedInt,
            MatroskaTag::ChapterPhysicalEquiv => SpecTagType::UnsignedInt,
            MatroskaTag::ChapterSegmentEditionUid => SpecTagType::UnsignedInt,
            MatroskaTag::ChapterUid => SpecTagType::UnsignedInt,
            MatroskaTag::EditionFlagOrdered => SpecTagType::UnsignedInt,
            MatroskaTag::EditionFlagDefault => SpecTagType::UnsignedInt,
            MatroskaTag::EditionFlagHidden => SpecTagType::UnsignedInt,
            MatroskaTag::EditionUid => SpecTagType::UnsignedInt,
            MatroskaTag::FileUid => SpecTagType::UnsignedInt,
            MatroskaTag::CueRefCodecState => SpecTagType::UnsignedInt,
            MatroskaTag::CueRefNumber => SpecTagType::UnsignedInt,
            MatroskaTag::CueCodecState => SpecTagType::UnsignedInt,
            MatroskaTag::CueDuration => SpecTagType::UnsignedInt,
            MatroskaTag::CueRelativePosition => SpecTagType::UnsignedInt,
            MatroskaTag::CueClusterPosition => SpecTagType::UnsignedInt,
            MatroskaTag::CueTrack => SpecTagType::UnsignedInt,
            MatroskaTag::CueTime => SpecTagType::UnsignedInt,
            MatroskaTag::AesSettingsCipherMode => SpecTagType::UnsignedInt,
            MatroskaTag::ContentSigHashAlgo => SpecTagType::UnsignedInt,
            MatroskaTag::ContentSigAlgo => SpecTagType::UnsignedInt,
            MatroskaTag::ContentEncAlgo => SpecTagType::UnsignedInt,
            MatroskaTag::TrickMasterTrackUid => SpecTagType::UnsignedInt,
            MatroskaTag::TrickTrackFlag => SpecTagType::UnsignedInt,
            MatroskaTag::TrickTrackUid => SpecTagType::UnsignedInt,
            MatroskaTag::TrackJoinUid => SpecTagType::UnsignedInt,
            MatroskaTag::TrackPlaneType => SpecTagType::UnsignedInt,
            MatroskaTag::TrackPlaneUid => SpecTagType::UnsignedInt,
            MatroskaTag::Channels => SpecTagType::UnsignedInt,
            MatroskaTag::AspectRatioType => SpecTagType::UnsignedInt,
            MatroskaTag::DisplayUnit => SpecTagType::UnsignedInt,
            MatroskaTag::DisplayHeight => SpecTagType::UnsignedInt,
            MatroskaTag::DisplayWidth => SpecTagType::UnsignedInt,
            MatroskaTag::PixelCropRight => SpecTagType::UnsignedInt,
            MatroskaTag::PixelCropLeft => SpecTagType::UnsignedInt,
            MatroskaTag::PixelCropTop => SpecTagType::UnsignedInt,
            MatroskaTag::PixelCropBottom => SpecTagType::UnsignedInt,
            MatroskaTag::PixelHeight => SpecTagType::UnsignedInt,
            MatroskaTag::PixelWidth => SpecTagType::UnsignedInt,
            MatroskaTag::OldStereoMode => SpecTagType::UnsignedInt,
            MatroskaTag::AlphaMode => SpecTagType::UnsignedInt,
            MatroskaTag::StereoMode => SpecTagType::UnsignedInt,
            MatroskaTag::FlagInterlaced => SpecTagType::UnsignedInt,
            MatroskaTag::TrackTranslateCodec => SpecTagType::UnsignedInt,
            MatroskaTag::TrackTranslateEditionUid => SpecTagType::UnsignedInt,
            MatroskaTag::SeekPreRoll => SpecTagType::UnsignedInt,
            MatroskaTag::CodecDelay => SpecTagType::UnsignedInt,
            MatroskaTag::TrackOverlay => SpecTagType::UnsignedInt,
            MatroskaTag::CodecDecodeAll => SpecTagType::UnsignedInt,
            MatroskaTag::MaxBlockAdditionId => SpecTagType::UnsignedInt,
            MatroskaTag::DefaultDecodedFieldDuration => SpecTagType::UnsignedInt,
            MatroskaTag::DefaultDuration => SpecTagType::UnsignedInt,
            MatroskaTag::MaxCache => SpecTagType::UnsignedInt,
            MatroskaTag::MinCache => SpecTagType::UnsignedInt,
            MatroskaTag::FlagLacing => SpecTagType::UnsignedInt,
            MatroskaTag::FlagForced => SpecTagType::UnsignedInt,
            MatroskaTag::FlagEnabled => SpecTagType::UnsignedInt,
            MatroskaTag::TrackUid => SpecTagType::UnsignedInt,
            MatroskaTag::TrackNumber => SpecTagType::UnsignedInt,
            MatroskaTag::ReferenceTimeCode => SpecTagType::UnsignedInt,
            MatroskaTag::ReferenceOffset => SpecTagType::UnsignedInt,
            MatroskaTag::SliceDuration => SpecTagType::UnsignedInt,
            MatroskaTag::Delay => SpecTagType::UnsignedInt,
            MatroskaTag::BlockAdditionId => SpecTagType::UnsignedInt,
            MatroskaTag::FrameNumber => SpecTagType::UnsignedInt,
            MatroskaTag::LaceNumber => SpecTagType::UnsignedInt,
            MatroskaTag::ReferencePriority => SpecTagType::UnsignedInt,
            MatroskaTag::BlockDuration => SpecTagType::UnsignedInt,
            MatroskaTag::BlockAddId => SpecTagType::UnsignedInt,
            MatroskaTag::PrevSize => SpecTagType::UnsignedInt,
            MatroskaTag::Position => SpecTagType::UnsignedInt,
            MatroskaTag::SilentTrackNumber => SpecTagType::UnsignedInt,
            MatroskaTag::Timecode => SpecTagType::UnsignedInt,
            MatroskaTag::TimecodeScaleDenominator => SpecTagType::UnsignedInt,
            MatroskaTag::TimecodeScale => SpecTagType::UnsignedInt,
            MatroskaTag::ChapterTranslateCodec => SpecTagType::UnsignedInt,
            MatroskaTag::ChapterTranslateEditionUid => SpecTagType::UnsignedInt,
            MatroskaTag::SeekPosition => SpecTagType::UnsignedInt,
            MatroskaTag::SignatureHash => SpecTagType::UnsignedInt,
            MatroskaTag::SignatureAlgo => SpecTagType::UnsignedInt,
            MatroskaTag::EbmlMaxSizeLength => SpecTagType::UnsignedInt,
            MatroskaTag::EbmlMaxIdLength => SpecTagType::UnsignedInt,
            MatroskaTag::EbmlReadVersion => SpecTagType::UnsignedInt,
            MatroskaTag::TrackOffset => SpecTagType::Integer,
            MatroskaTag::DiscardPadding => SpecTagType::Integer,
            MatroskaTag::ReferenceVirtual => SpecTagType::Integer,
            MatroskaTag::ReferenceBlock => SpecTagType::Integer,
            MatroskaTag::CodecId => SpecTagType::Utf8,
            MatroskaTag::DocType => SpecTagType::Utf8,
            MatroskaTag::FileMimeType => SpecTagType::Utf8,
            MatroskaTag::TagLanguage => SpecTagType::Utf8,
            MatroskaTag::TargetType => SpecTagType::Utf8,
            MatroskaTag::ChapCountry => SpecTagType::Utf8,
            MatroskaTag::ChapLanguage => SpecTagType::Utf8,
            MatroskaTag::CodecDownloadUrl => SpecTagType::Utf8,
            MatroskaTag::CodecInfoUrl => SpecTagType::Utf8,
            MatroskaTag::Language => SpecTagType::Utf8,
            MatroskaTag::ChapString => SpecTagType::Utf8,
            MatroskaTag::TagString => SpecTagType::Utf8,
            MatroskaTag::ChapterStringUid => SpecTagType::Utf8,
            MatroskaTag::WritingApp => SpecTagType::Utf8,
            MatroskaTag::SegmentFilename => SpecTagType::Utf8,
            MatroskaTag::CodecName => SpecTagType::Utf8,
            MatroskaTag::TagName => SpecTagType::Utf8,
            MatroskaTag::FileName => SpecTagType::Utf8,
            MatroskaTag::FileDescription => SpecTagType::Utf8,
            MatroskaTag::CodecSettings => SpecTagType::Utf8,
            MatroskaTag::Name => SpecTagType::Utf8,
            MatroskaTag::MuxingApp => SpecTagType::Utf8,
            MatroskaTag::Title => SpecTagType::Utf8,
            MatroskaTag::NextFilename => SpecTagType::Utf8,
            MatroskaTag::PrevFilename => SpecTagType::Utf8,
            MatroskaTag::ContentCompSettings => SpecTagType::Binary,
            MatroskaTag::SegmentFamily => SpecTagType::Binary,
            MatroskaTag::TagBinary => SpecTagType::Binary,
            MatroskaTag::FileReferral => SpecTagType::Binary,
            MatroskaTag::SignedElement => SpecTagType::Binary,
            MatroskaTag::ChapProcessData => SpecTagType::Binary,
            MatroskaTag::ChapProcessPrivate => SpecTagType::Binary,
            MatroskaTag::ChapterSegmentUid => SpecTagType::Binary,
            MatroskaTag::FileData => SpecTagType::Binary,
            MatroskaTag::ContentSigKeyId => SpecTagType::Binary,
            MatroskaTag::ContentSignature => SpecTagType::Binary,
            MatroskaTag::ContentEncKeyId => SpecTagType::Binary,
            MatroskaTag::TrickMasterTrackSegmentUid => SpecTagType::Binary,
            MatroskaTag::TrickTrackSegmentUid => SpecTagType::Binary,
            MatroskaTag::ChannelPositions => SpecTagType::Binary,
            MatroskaTag::ColourSpace => SpecTagType::Binary,
            MatroskaTag::TrackTranslateTrackId => SpecTagType::Binary,
            MatroskaTag::CodecPrivate => SpecTagType::Binary,
            MatroskaTag::EncryptedBlock => SpecTagType::Binary,
            MatroskaTag::CodecState => SpecTagType::Binary,
            MatroskaTag::BlockAdditional => SpecTagType::Binary,
            MatroskaTag::BlockVirtual => SpecTagType::Binary,
            MatroskaTag::ChapterTranslateId => SpecTagType::Binary,
            MatroskaTag::NextUid => SpecTagType::Binary,
            MatroskaTag::PrevUid => SpecTagType::Binary,
            MatroskaTag::SegmentUid => SpecTagType::Binary,
            MatroskaTag::SeekId => SpecTagType::Binary,
            MatroskaTag::Signature => SpecTagType::Binary,
            MatroskaTag::SignaturePublicKey => SpecTagType::Binary,
            MatroskaTag::Crc32 => SpecTagType::Binary,
            MatroskaTag::Void => SpecTagType::Binary,
            MatroskaTag::DateUtc => SpecTagType::Binary,
            MatroskaTag::Duration => SpecTagType::Float,
            MatroskaTag::OutputSamplingFrequency => SpecTagType::Float,
            MatroskaTag::SamplingFrequency => SpecTagType::Float,
            MatroskaTag::FrameRate => SpecTagType::Float,
            MatroskaTag::GammaValue => SpecTagType::Float,
            MatroskaTag::TrackTimecodeScale => SpecTagType::Float,

            MatroskaTag::Unknown => SpecTagType::Binary,
        }
    }
}