use super::Ebml;
use super::ids::EbmlId;
use super::ElementSize;

#[derive(Debug, Clone, Default)]
pub struct EbmlHeader {
    pub size: u64,

    pub version: Ebml<u64>,
    pub read_version: Ebml<u64>,
    pub doc_type: Ebml<String>,
    pub doc_type_version: Ebml<u64>,
    pub doc_type_read_version: Ebml<u64>,
    pub doc_type_extension: Option<Ebml<DocTypeExtension>>,
    pub doc_type_extension_name: Option<Ebml<String>>,
    pub doc_type_extension_version: Option<Ebml<u64>>,
    pub ebml_max_id_length: Ebml<u64>,
    pub ebml_max_size_length: Ebml<u64>,
}
impl EbmlHeader {
    pub fn elements(&self) -> std::collections::BTreeSet<EbmlHeaderFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(EbmlHeaderFields::Version(self.version.clone()));
        elements.insert(EbmlHeaderFields::ReadVersion(self.read_version.clone()));
        elements.insert(EbmlHeaderFields::DocType(self.doc_type.clone()));
        elements.insert(EbmlHeaderFields::DocTypeVersion(self.doc_type_version.clone()));
        elements.insert(EbmlHeaderFields::DocTypeReadVersion(self.doc_type_read_version.clone()));
        if let Some(el) = &self.doc_type_extension { elements.insert(EbmlHeaderFields::DocTypeExtension(el.clone())); }
        if let Some(el) = &self.doc_type_extension_name { elements.insert(EbmlHeaderFields::DocTypeExtensionName(el.clone())); }
        if let Some(el) = &self.doc_type_extension_version { elements.insert(EbmlHeaderFields::DocTypeExtensionVersion(el.clone())); }
        elements.insert(EbmlHeaderFields::EbmlMaxIdLength(self.ebml_max_id_length.clone()));
        elements.insert(EbmlHeaderFields::EbmlMaxSizeLength(self.ebml_max_size_length.clone()));
        elements
    }
}
#[derive(Debug)]
pub enum EbmlHeaderFields {
    Version(Ebml<u64>),
    ReadVersion(Ebml<u64>),
    DocType(Ebml<String>),
    DocTypeVersion(Ebml<u64>),
    DocTypeReadVersion(Ebml<u64>),
    DocTypeExtension(Ebml<DocTypeExtension>),
    DocTypeExtensionName(Ebml<String>),
    DocTypeExtensionVersion(Ebml<u64>),
    EbmlMaxIdLength(Ebml<u64>),
    EbmlMaxSizeLength(Ebml<u64>),
}
impl EbmlHeaderFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::Version(val) => (EbmlId::Version, val.index, val.id),
            Self::ReadVersion(val) => (EbmlId::ReadVersion, val.index, val.id),
            Self::DocType(val) => (EbmlId::DocType, val.index, val.id),
            Self::DocTypeVersion(val) => (EbmlId::DocTypeVersion, val.index, val.id),
            Self::DocTypeReadVersion(val) => (EbmlId::DocTypeReadVersion, val.index, val.id),
            Self::DocTypeExtension(val) => (EbmlId::DocTypeExtension, val.index, val.id),
            Self::DocTypeExtensionName(val) => (EbmlId::DocTypeExtensionName, val.index, val.id),
            Self::DocTypeExtensionVersion(val) => (EbmlId::DocTypeExtensionVersion, val.index, val.id),
            Self::EbmlMaxIdLength(val) => (EbmlId::EbmlMaxIdLength, val.index, val.id),
            Self::EbmlMaxSizeLength(val) => (EbmlId::EbmlMaxSizeLength, val.index, val.id),
        }
    }
}
crate::impl_ord!(EbmlHeaderFields);

#[derive(Debug, Clone, Default)]
pub struct DocTypeExtension {
    pub size: u64,

}

#[derive(Debug, Clone, Default)]
pub struct Segment {
    pub size: ElementSize,

    pub void: Vec<Ebml<Vec<u8>>>,
    pub crc_32: Option<Ebml<Vec<u8>>>,
    pub seek_head: Vec<Ebml<SeekHead>>,
    pub info: Ebml<Info>,
    pub cluster: Vec<Ebml<Cluster>>,
    pub tracks: Option<Ebml<Tracks>>,
    pub cues: Option<Ebml<Cues>>,
    pub attachments: Option<Ebml<Attachments>>,
    pub chapters: Option<Ebml<Chapters>>,
    pub tags: Vec<Ebml<Tags>>,
}
impl Segment {
    pub fn elements(&self) -> std::collections::BTreeSet<SegmentFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.void { elements.insert(SegmentFields::Void(el.clone())); }
        if let Some(el) = &self.crc_32 { elements.insert(SegmentFields::Crc32(el.clone())); }
        for el in &self.seek_head { elements.insert(SegmentFields::SeekHead(el.clone())); }
        elements.insert(SegmentFields::Info(self.info.clone()));
        for el in &self.cluster { elements.insert(SegmentFields::Cluster(el.clone())); }
        if let Some(el) = &self.tracks { elements.insert(SegmentFields::Tracks(el.clone())); }
        if let Some(el) = &self.cues { elements.insert(SegmentFields::Cues(el.clone())); }
        if let Some(el) = &self.attachments { elements.insert(SegmentFields::Attachments(el.clone())); }
        if let Some(el) = &self.chapters { elements.insert(SegmentFields::Chapters(el.clone())); }
        for el in &self.tags { elements.insert(SegmentFields::Tags(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum SegmentFields {
    Void(Ebml<Vec<u8>>),
    Crc32(Ebml<Vec<u8>>),
    SeekHead(Ebml<SeekHead>),
    Info(Ebml<Info>),
    Cluster(Ebml<Cluster>),
    Tracks(Ebml<Tracks>),
    Cues(Ebml<Cues>),
    Attachments(Ebml<Attachments>),
    Chapters(Ebml<Chapters>),
    Tags(Ebml<Tags>),
}
impl SegmentFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::Void(val) => (EbmlId::Void, val.index, val.id),
            Self::Crc32(val) => (EbmlId::Crc32, val.index, val.id),
            Self::SeekHead(val) => (EbmlId::SeekHead, val.index, val.id),
            Self::Info(val) => (EbmlId::Info, val.index, val.id),
            Self::Cluster(val) => (EbmlId::Cluster, val.index, val.id),
            Self::Tracks(val) => (EbmlId::Tracks, val.index, val.id),
            Self::Cues(val) => (EbmlId::Cues, val.index, val.id),
            Self::Attachments(val) => (EbmlId::Attachments, val.index, val.id),
            Self::Chapters(val) => (EbmlId::Chapters, val.index, val.id),
            Self::Tags(val) => (EbmlId::Tags, val.index, val.id),
        }
    }
}
crate::impl_ord!(SegmentFields);

#[derive(Debug, Clone, Default)]
pub struct SeekHead {
    pub size: u64,

    pub seek: Vec<Ebml<Seek>>,
}
impl SeekHead {
    pub fn elements(&self) -> std::collections::BTreeSet<SeekHeadFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.seek { elements.insert(SeekHeadFields::Seek(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum SeekHeadFields {
    Seek(Ebml<Seek>),
}
impl SeekHeadFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::Seek(val) => (EbmlId::Seek, val.index, val.id),
        }
    }
}
crate::impl_ord!(SeekHeadFields);

#[derive(Debug, Clone, Default)]
pub struct Seek {
    pub size: u64,

    pub seek_id: Ebml<Vec<u8>>,
    pub seek_position: Ebml<u64>,
}
impl Seek {
    pub fn elements(&self) -> std::collections::BTreeSet<SeekFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(SeekFields::SeekId(self.seek_id.clone()));
        elements.insert(SeekFields::SeekPosition(self.seek_position.clone()));
        elements
    }
}
#[derive(Debug)]
pub enum SeekFields {
    SeekId(Ebml<Vec<u8>>),
    SeekPosition(Ebml<u64>),
}
impl SeekFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::SeekId(val) => (EbmlId::SeekId, val.index, val.id),
            Self::SeekPosition(val) => (EbmlId::SeekPosition, val.index, val.id),
        }
    }
}
crate::impl_ord!(SeekFields);

#[derive(Debug, Clone, Default)]
pub struct Info {
    pub size: u64,

    pub segment_uuid: Option<Ebml<Vec<u8>>>,
    pub segment_filename: Option<Ebml<String>>,
    pub prev_uuid: Option<Ebml<Vec<u8>>>,
    pub prev_filename: Option<Ebml<String>>,
    pub next_uuid: Option<Ebml<Vec<u8>>>,
    pub next_filename: Option<Ebml<String>>,
    pub segment_family: Vec<Ebml<Vec<u8>>>,
    pub chapter_translate: Vec<Ebml<ChapterTranslate>>,
    pub timestamp_scale: Ebml<u64>,
    pub duration: Option<Ebml<f64>>,
    pub date_utc: Option<Ebml<time::OffsetDateTime>>,
    pub title: Option<Ebml<String>>,
    pub muxing_app: Ebml<String>,
    pub writing_app: Ebml<String>,
}
impl Info {
    pub fn elements(&self) -> std::collections::BTreeSet<InfoFields> {
        let mut elements = std::collections::BTreeSet::new();
        if let Some(el) = &self.segment_uuid { elements.insert(InfoFields::SegmentUuid(el.clone())); }
        if let Some(el) = &self.segment_filename { elements.insert(InfoFields::SegmentFilename(el.clone())); }
        if let Some(el) = &self.prev_uuid { elements.insert(InfoFields::PrevUuid(el.clone())); }
        if let Some(el) = &self.prev_filename { elements.insert(InfoFields::PrevFilename(el.clone())); }
        if let Some(el) = &self.next_uuid { elements.insert(InfoFields::NextUuid(el.clone())); }
        if let Some(el) = &self.next_filename { elements.insert(InfoFields::NextFilename(el.clone())); }
        for el in &self.segment_family { elements.insert(InfoFields::SegmentFamily(el.clone())); }
        for el in &self.chapter_translate { elements.insert(InfoFields::ChapterTranslate(el.clone())); }
        elements.insert(InfoFields::TimestampScale(self.timestamp_scale.clone()));
        if let Some(el) = &self.duration { elements.insert(InfoFields::Duration(el.clone())); }
        if let Some(el) = &self.date_utc { elements.insert(InfoFields::DateUtc(el.clone())); }
        if let Some(el) = &self.title { elements.insert(InfoFields::Title(el.clone())); }
        elements.insert(InfoFields::MuxingApp(self.muxing_app.clone()));
        elements.insert(InfoFields::WritingApp(self.writing_app.clone()));
        elements
    }
}
#[derive(Debug)]
pub enum InfoFields {
    SegmentUuid(Ebml<Vec<u8>>),
    SegmentFilename(Ebml<String>),
    PrevUuid(Ebml<Vec<u8>>),
    PrevFilename(Ebml<String>),
    NextUuid(Ebml<Vec<u8>>),
    NextFilename(Ebml<String>),
    SegmentFamily(Ebml<Vec<u8>>),
    ChapterTranslate(Ebml<ChapterTranslate>),
    TimestampScale(Ebml<u64>),
    Duration(Ebml<f64>),
    DateUtc(Ebml<time::OffsetDateTime>),
    Title(Ebml<String>),
    MuxingApp(Ebml<String>),
    WritingApp(Ebml<String>),
}
impl InfoFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::SegmentUuid(val) => (EbmlId::SegmentUuid, val.index, val.id),
            Self::SegmentFilename(val) => (EbmlId::SegmentFilename, val.index, val.id),
            Self::PrevUuid(val) => (EbmlId::PrevUuid, val.index, val.id),
            Self::PrevFilename(val) => (EbmlId::PrevFilename, val.index, val.id),
            Self::NextUuid(val) => (EbmlId::NextUuid, val.index, val.id),
            Self::NextFilename(val) => (EbmlId::NextFilename, val.index, val.id),
            Self::SegmentFamily(val) => (EbmlId::SegmentFamily, val.index, val.id),
            Self::ChapterTranslate(val) => (EbmlId::ChapterTranslate, val.index, val.id),
            Self::TimestampScale(val) => (EbmlId::TimestampScale, val.index, val.id),
            Self::Duration(val) => (EbmlId::Duration, val.index, val.id),
            Self::DateUtc(val) => (EbmlId::DateUtc, val.index, val.id),
            Self::Title(val) => (EbmlId::Title, val.index, val.id),
            Self::MuxingApp(val) => (EbmlId::MuxingApp, val.index, val.id),
            Self::WritingApp(val) => (EbmlId::WritingApp, val.index, val.id),
        }
    }
}
crate::impl_ord!(InfoFields);

#[derive(Debug, Clone, Default)]
pub struct ChapterTranslate {
    pub size: u64,

    pub chapter_translate_id: Ebml<Vec<u8>>,
    pub chapter_translate_codec: Ebml<u64>,
    pub chapter_translate_edition_uid: Vec<Ebml<u64>>,
}
impl ChapterTranslate {
    pub fn elements(&self) -> std::collections::BTreeSet<ChapterTranslateFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ChapterTranslateFields::ChapterTranslateId(self.chapter_translate_id.clone()));
        elements.insert(ChapterTranslateFields::ChapterTranslateCodec(self.chapter_translate_codec.clone()));
        for el in &self.chapter_translate_edition_uid { elements.insert(ChapterTranslateFields::ChapterTranslateEditionUid(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ChapterTranslateFields {
    ChapterTranslateId(Ebml<Vec<u8>>),
    ChapterTranslateCodec(Ebml<u64>),
    ChapterTranslateEditionUid(Ebml<u64>),
}
impl ChapterTranslateFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ChapterTranslateId(val) => (EbmlId::ChapterTranslateId, val.index, val.id),
            Self::ChapterTranslateCodec(val) => (EbmlId::ChapterTranslateCodec, val.index, val.id),
            Self::ChapterTranslateEditionUid(val) => (EbmlId::ChapterTranslateEditionUid, val.index, val.id),
        }
    }
}
crate::impl_ord!(ChapterTranslateFields);

#[derive(Debug, Clone, Default)]
pub struct Cluster {
    pub size: ElementSize,

    pub timestamp: Ebml<u64>,
    pub silent_tracks: Option<Ebml<SilentTracks>>,
    pub position: Option<Ebml<u64>>,
    pub prev_size: Option<Ebml<u64>>,
    pub simple_block: Vec<Ebml<Vec<u8>>>,
    pub block_group: Vec<Ebml<BlockGroup>>,
    pub encrypted_block: Vec<Ebml<Vec<u8>>>,
}
impl Cluster {
    pub fn elements(&self) -> std::collections::BTreeSet<ClusterFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ClusterFields::Timestamp(self.timestamp.clone()));
        if let Some(el) = &self.silent_tracks { elements.insert(ClusterFields::SilentTracks(el.clone())); }
        if let Some(el) = &self.position { elements.insert(ClusterFields::Position(el.clone())); }
        if let Some(el) = &self.prev_size { elements.insert(ClusterFields::PrevSize(el.clone())); }
        for el in &self.simple_block { elements.insert(ClusterFields::SimpleBlock(el.clone())); }
        for el in &self.block_group { elements.insert(ClusterFields::BlockGroup(el.clone())); }
        for el in &self.encrypted_block { elements.insert(ClusterFields::EncryptedBlock(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ClusterFields {
    Timestamp(Ebml<u64>),
    SilentTracks(Ebml<SilentTracks>),
    Position(Ebml<u64>),
    PrevSize(Ebml<u64>),
    SimpleBlock(Ebml<Vec<u8>>),
    BlockGroup(Ebml<BlockGroup>),
    EncryptedBlock(Ebml<Vec<u8>>),
}
impl ClusterFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::Timestamp(val) => (EbmlId::Timestamp, val.index, val.id),
            Self::SilentTracks(val) => (EbmlId::SilentTracks, val.index, val.id),
            Self::Position(val) => (EbmlId::Position, val.index, val.id),
            Self::PrevSize(val) => (EbmlId::PrevSize, val.index, val.id),
            Self::SimpleBlock(val) => (EbmlId::SimpleBlock, val.index, val.id),
            Self::BlockGroup(val) => (EbmlId::BlockGroup, val.index, val.id),
            Self::EncryptedBlock(val) => (EbmlId::EncryptedBlock, val.index, val.id),
        }
    }
}
crate::impl_ord!(ClusterFields);

#[derive(Debug, Clone, Default)]
pub struct SilentTracks {
    pub size: u64,

    pub silent_track_number: Vec<Ebml<u64>>,
}
impl SilentTracks {
    pub fn elements(&self) -> std::collections::BTreeSet<SilentTracksFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.silent_track_number { elements.insert(SilentTracksFields::SilentTrackNumber(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum SilentTracksFields {
    SilentTrackNumber(Ebml<u64>),
}
impl SilentTracksFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::SilentTrackNumber(val) => (EbmlId::SilentTrackNumber, val.index, val.id),
        }
    }
}
crate::impl_ord!(SilentTracksFields);

#[derive(Debug, Clone, Default)]
pub struct BlockGroup {
    pub size: u64,

    pub block: Ebml<Vec<u8>>,
    pub block_virtual: Option<Ebml<Vec<u8>>>,
    pub block_additions: Option<Ebml<BlockAdditions>>,
    pub block_duration: Option<Ebml<u64>>,
    pub reference_priority: Ebml<u64>,
    pub reference_block: Vec<Ebml<i64>>,
    pub reference_virtual: Option<Ebml<i64>>,
    pub codec_state: Option<Ebml<Vec<u8>>>,
    pub discard_padding: Option<Ebml<i64>>,
    pub slices: Option<Ebml<Slices>>,
    pub reference_frame: Option<Ebml<ReferenceFrame>>,
}
impl BlockGroup {
    pub fn elements(&self) -> std::collections::BTreeSet<BlockGroupFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(BlockGroupFields::Block(self.block.clone()));
        if let Some(el) = &self.block_virtual { elements.insert(BlockGroupFields::BlockVirtual(el.clone())); }
        if let Some(el) = &self.block_additions { elements.insert(BlockGroupFields::BlockAdditions(el.clone())); }
        if let Some(el) = &self.block_duration { elements.insert(BlockGroupFields::BlockDuration(el.clone())); }
        elements.insert(BlockGroupFields::ReferencePriority(self.reference_priority.clone()));
        for el in &self.reference_block { elements.insert(BlockGroupFields::ReferenceBlock(el.clone())); }
        if let Some(el) = &self.reference_virtual { elements.insert(BlockGroupFields::ReferenceVirtual(el.clone())); }
        if let Some(el) = &self.codec_state { elements.insert(BlockGroupFields::CodecState(el.clone())); }
        if let Some(el) = &self.discard_padding { elements.insert(BlockGroupFields::DiscardPadding(el.clone())); }
        if let Some(el) = &self.slices { elements.insert(BlockGroupFields::Slices(el.clone())); }
        if let Some(el) = &self.reference_frame { elements.insert(BlockGroupFields::ReferenceFrame(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum BlockGroupFields {
    Block(Ebml<Vec<u8>>),
    BlockVirtual(Ebml<Vec<u8>>),
    BlockAdditions(Ebml<BlockAdditions>),
    BlockDuration(Ebml<u64>),
    ReferencePriority(Ebml<u64>),
    ReferenceBlock(Ebml<i64>),
    ReferenceVirtual(Ebml<i64>),
    CodecState(Ebml<Vec<u8>>),
    DiscardPadding(Ebml<i64>),
    Slices(Ebml<Slices>),
    ReferenceFrame(Ebml<ReferenceFrame>),
}
impl BlockGroupFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::Block(val) => (EbmlId::Block, val.index, val.id),
            Self::BlockVirtual(val) => (EbmlId::BlockVirtual, val.index, val.id),
            Self::BlockAdditions(val) => (EbmlId::BlockAdditions, val.index, val.id),
            Self::BlockDuration(val) => (EbmlId::BlockDuration, val.index, val.id),
            Self::ReferencePriority(val) => (EbmlId::ReferencePriority, val.index, val.id),
            Self::ReferenceBlock(val) => (EbmlId::ReferenceBlock, val.index, val.id),
            Self::ReferenceVirtual(val) => (EbmlId::ReferenceVirtual, val.index, val.id),
            Self::CodecState(val) => (EbmlId::CodecState, val.index, val.id),
            Self::DiscardPadding(val) => (EbmlId::DiscardPadding, val.index, val.id),
            Self::Slices(val) => (EbmlId::Slices, val.index, val.id),
            Self::ReferenceFrame(val) => (EbmlId::ReferenceFrame, val.index, val.id),
        }
    }
}
crate::impl_ord!(BlockGroupFields);

#[derive(Debug, Clone, Default)]
pub struct BlockAdditions {
    pub size: u64,

    pub block_more: Vec<Ebml<BlockMore>>,
}
impl BlockAdditions {
    pub fn elements(&self) -> std::collections::BTreeSet<BlockAdditionsFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.block_more { elements.insert(BlockAdditionsFields::BlockMore(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum BlockAdditionsFields {
    BlockMore(Ebml<BlockMore>),
}
impl BlockAdditionsFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::BlockMore(val) => (EbmlId::BlockMore, val.index, val.id),
        }
    }
}
crate::impl_ord!(BlockAdditionsFields);

#[derive(Debug, Clone, Default)]
pub struct BlockMore {
    pub size: u64,

    pub block_additional: Ebml<Vec<u8>>,
    pub block_add_id: Ebml<u64>,
}
impl BlockMore {
    pub fn elements(&self) -> std::collections::BTreeSet<BlockMoreFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(BlockMoreFields::BlockAdditional(self.block_additional.clone()));
        elements.insert(BlockMoreFields::BlockAddId(self.block_add_id.clone()));
        elements
    }
}
#[derive(Debug)]
pub enum BlockMoreFields {
    BlockAdditional(Ebml<Vec<u8>>),
    BlockAddId(Ebml<u64>),
}
impl BlockMoreFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::BlockAdditional(val) => (EbmlId::BlockAdditional, val.index, val.id),
            Self::BlockAddId(val) => (EbmlId::BlockAddId, val.index, val.id),
        }
    }
}
crate::impl_ord!(BlockMoreFields);

#[derive(Debug, Clone, Default)]
pub struct Slices {
    pub size: u64,

    pub time_slice: Vec<Ebml<TimeSlice>>,
}
impl Slices {
    pub fn elements(&self) -> std::collections::BTreeSet<SlicesFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.time_slice { elements.insert(SlicesFields::TimeSlice(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum SlicesFields {
    TimeSlice(Ebml<TimeSlice>),
}
impl SlicesFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::TimeSlice(val) => (EbmlId::TimeSlice, val.index, val.id),
        }
    }
}
crate::impl_ord!(SlicesFields);

#[derive(Debug, Clone, Default)]
pub struct TimeSlice {
    pub size: u64,

    pub lace_number: Option<Ebml<u64>>,
    pub frame_number: Option<Ebml<u64>>,
    pub block_addition_id: Option<Ebml<u64>>,
    pub delay: Option<Ebml<u64>>,
    pub slice_duration: Option<Ebml<u64>>,
}
impl TimeSlice {
    pub fn elements(&self) -> std::collections::BTreeSet<TimeSliceFields> {
        let mut elements = std::collections::BTreeSet::new();
        if let Some(el) = &self.lace_number { elements.insert(TimeSliceFields::LaceNumber(el.clone())); }
        if let Some(el) = &self.frame_number { elements.insert(TimeSliceFields::FrameNumber(el.clone())); }
        if let Some(el) = &self.block_addition_id { elements.insert(TimeSliceFields::BlockAdditionId(el.clone())); }
        if let Some(el) = &self.delay { elements.insert(TimeSliceFields::Delay(el.clone())); }
        if let Some(el) = &self.slice_duration { elements.insert(TimeSliceFields::SliceDuration(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum TimeSliceFields {
    LaceNumber(Ebml<u64>),
    FrameNumber(Ebml<u64>),
    BlockAdditionId(Ebml<u64>),
    Delay(Ebml<u64>),
    SliceDuration(Ebml<u64>),
}
impl TimeSliceFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::LaceNumber(val) => (EbmlId::LaceNumber, val.index, val.id),
            Self::FrameNumber(val) => (EbmlId::FrameNumber, val.index, val.id),
            Self::BlockAdditionId(val) => (EbmlId::BlockAdditionId, val.index, val.id),
            Self::Delay(val) => (EbmlId::Delay, val.index, val.id),
            Self::SliceDuration(val) => (EbmlId::SliceDuration, val.index, val.id),
        }
    }
}
crate::impl_ord!(TimeSliceFields);

#[derive(Debug, Clone, Default)]
pub struct ReferenceFrame {
    pub size: u64,

    pub reference_offset: Ebml<u64>,
    pub reference_timestamp: Ebml<u64>,
}
impl ReferenceFrame {
    pub fn elements(&self) -> std::collections::BTreeSet<ReferenceFrameFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ReferenceFrameFields::ReferenceOffset(self.reference_offset.clone()));
        elements.insert(ReferenceFrameFields::ReferenceTimestamp(self.reference_timestamp.clone()));
        elements
    }
}
#[derive(Debug)]
pub enum ReferenceFrameFields {
    ReferenceOffset(Ebml<u64>),
    ReferenceTimestamp(Ebml<u64>),
}
impl ReferenceFrameFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ReferenceOffset(val) => (EbmlId::ReferenceOffset, val.index, val.id),
            Self::ReferenceTimestamp(val) => (EbmlId::ReferenceTimestamp, val.index, val.id),
        }
    }
}
crate::impl_ord!(ReferenceFrameFields);

#[derive(Debug, Clone, Default)]
pub struct Tracks {
    pub size: u64,

    pub track_entry: Vec<Ebml<TrackEntry>>,
}
impl Tracks {
    pub fn elements(&self) -> std::collections::BTreeSet<TracksFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.track_entry { elements.insert(TracksFields::TrackEntry(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum TracksFields {
    TrackEntry(Ebml<TrackEntry>),
}
impl TracksFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::TrackEntry(val) => (EbmlId::TrackEntry, val.index, val.id),
        }
    }
}
crate::impl_ord!(TracksFields);

#[derive(Debug, Clone, Default)]
pub struct TrackEntry {
    pub size: u64,

    pub track_number: Ebml<u64>,
    pub track_uid: Ebml<u64>,
    pub track_type: Ebml<u64>,
    pub flag_enabled: Ebml<u64>,
    pub flag_default: Ebml<u64>,
    pub flag_forced: Ebml<u64>,
    pub flag_hearing_impaired: Option<Ebml<u64>>,
    pub flag_visual_impaired: Option<Ebml<u64>>,
    pub flag_text_descriptions: Option<Ebml<u64>>,
    pub flag_original: Option<Ebml<u64>>,
    pub flag_commentary: Option<Ebml<u64>>,
    pub flag_lacing: Ebml<u64>,
    pub min_cache: Ebml<u64>,
    pub max_cache: Option<Ebml<u64>>,
    pub default_duration: Option<Ebml<u64>>,
    pub default_decoded_field_duration: Option<Ebml<u64>>,
    pub track_timestamp_scale: Ebml<f64>,
    pub track_offset: Option<Ebml<i64>>,
    pub max_block_addition_id: Ebml<u64>,
    pub block_addition_mapping: Vec<Ebml<BlockAdditionMapping>>,
    pub name: Option<Ebml<String>>,
    pub language: Ebml<String>,
    pub language_bcp_47: Option<Ebml<String>>,
    pub codec_id: Ebml<String>,
    pub codec_private: Option<Ebml<Vec<u8>>>,
    pub codec_name: Option<Ebml<String>>,
    pub attachment_link: Option<Ebml<u64>>,
    pub codec_settings: Option<Ebml<String>>,
    pub codec_info_url: Vec<Ebml<String>>,
    pub codec_download_url: Vec<Ebml<String>>,
    pub codec_decode_all: Ebml<u64>,
    pub track_overlay: Vec<Ebml<u64>>,
    pub codec_delay: Ebml<u64>,
    pub seek_pre_roll: Ebml<u64>,
    pub track_translate: Vec<Ebml<TrackTranslate>>,
    pub video: Option<Ebml<Video>>,
    pub audio: Option<Ebml<Audio>>,
    pub track_operation: Option<Ebml<TrackOperation>>,
    pub trick_track_uid: Option<Ebml<u64>>,
    pub trick_track_segment_uid: Option<Ebml<Vec<u8>>>,
    pub trick_track_flag: Option<Ebml<u64>>,
    pub trick_struct_track_uid: Option<Ebml<u64>>,
    pub trick_struct_track_segment_uid: Option<Ebml<Vec<u8>>>,
    pub content_encodings: Option<Ebml<ContentEncodings>>,
}
impl TrackEntry {
    pub fn elements(&self) -> std::collections::BTreeSet<TrackEntryFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(TrackEntryFields::TrackNumber(self.track_number.clone()));
        elements.insert(TrackEntryFields::TrackUid(self.track_uid.clone()));
        elements.insert(TrackEntryFields::TrackType(self.track_type.clone()));
        elements.insert(TrackEntryFields::FlagEnabled(self.flag_enabled.clone()));
        elements.insert(TrackEntryFields::FlagDefault(self.flag_default.clone()));
        elements.insert(TrackEntryFields::FlagForced(self.flag_forced.clone()));
        if let Some(el) = &self.flag_hearing_impaired { elements.insert(TrackEntryFields::FlagHearingImpaired(el.clone())); }
        if let Some(el) = &self.flag_visual_impaired { elements.insert(TrackEntryFields::FlagVisualImpaired(el.clone())); }
        if let Some(el) = &self.flag_text_descriptions { elements.insert(TrackEntryFields::FlagTextDescriptions(el.clone())); }
        if let Some(el) = &self.flag_original { elements.insert(TrackEntryFields::FlagOriginal(el.clone())); }
        if let Some(el) = &self.flag_commentary { elements.insert(TrackEntryFields::FlagCommentary(el.clone())); }
        elements.insert(TrackEntryFields::FlagLacing(self.flag_lacing.clone()));
        elements.insert(TrackEntryFields::MinCache(self.min_cache.clone()));
        if let Some(el) = &self.max_cache { elements.insert(TrackEntryFields::MaxCache(el.clone())); }
        if let Some(el) = &self.default_duration { elements.insert(TrackEntryFields::DefaultDuration(el.clone())); }
        if let Some(el) = &self.default_decoded_field_duration { elements.insert(TrackEntryFields::DefaultDecodedFieldDuration(el.clone())); }
        elements.insert(TrackEntryFields::TrackTimestampScale(self.track_timestamp_scale.clone()));
        if let Some(el) = &self.track_offset { elements.insert(TrackEntryFields::TrackOffset(el.clone())); }
        elements.insert(TrackEntryFields::MaxBlockAdditionId(self.max_block_addition_id.clone()));
        for el in &self.block_addition_mapping { elements.insert(TrackEntryFields::BlockAdditionMapping(el.clone())); }
        if let Some(el) = &self.name { elements.insert(TrackEntryFields::Name(el.clone())); }
        elements.insert(TrackEntryFields::Language(self.language.clone()));
        if let Some(el) = &self.language_bcp_47 { elements.insert(TrackEntryFields::LanguageBcp47(el.clone())); }
        elements.insert(TrackEntryFields::CodecId(self.codec_id.clone()));
        if let Some(el) = &self.codec_private { elements.insert(TrackEntryFields::CodecPrivate(el.clone())); }
        if let Some(el) = &self.codec_name { elements.insert(TrackEntryFields::CodecName(el.clone())); }
        if let Some(el) = &self.attachment_link { elements.insert(TrackEntryFields::AttachmentLink(el.clone())); }
        if let Some(el) = &self.codec_settings { elements.insert(TrackEntryFields::CodecSettings(el.clone())); }
        for el in &self.codec_info_url { elements.insert(TrackEntryFields::CodecInfoUrl(el.clone())); }
        for el in &self.codec_download_url { elements.insert(TrackEntryFields::CodecDownloadUrl(el.clone())); }
        elements.insert(TrackEntryFields::CodecDecodeAll(self.codec_decode_all.clone()));
        for el in &self.track_overlay { elements.insert(TrackEntryFields::TrackOverlay(el.clone())); }
        elements.insert(TrackEntryFields::CodecDelay(self.codec_delay.clone()));
        elements.insert(TrackEntryFields::SeekPreRoll(self.seek_pre_roll.clone()));
        for el in &self.track_translate { elements.insert(TrackEntryFields::TrackTranslate(el.clone())); }
        if let Some(el) = &self.video { elements.insert(TrackEntryFields::Video(el.clone())); }
        if let Some(el) = &self.audio { elements.insert(TrackEntryFields::Audio(el.clone())); }
        if let Some(el) = &self.track_operation { elements.insert(TrackEntryFields::TrackOperation(el.clone())); }
        if let Some(el) = &self.trick_track_uid { elements.insert(TrackEntryFields::TrickTrackUid(el.clone())); }
        if let Some(el) = &self.trick_track_segment_uid { elements.insert(TrackEntryFields::TrickTrackSegmentUid(el.clone())); }
        if let Some(el) = &self.trick_track_flag { elements.insert(TrackEntryFields::TrickTrackFlag(el.clone())); }
        if let Some(el) = &self.trick_struct_track_uid { elements.insert(TrackEntryFields::TrickStructTrackUid(el.clone())); }
        if let Some(el) = &self.trick_struct_track_segment_uid { elements.insert(TrackEntryFields::TrickStructTrackSegmentUid(el.clone())); }
        if let Some(el) = &self.content_encodings { elements.insert(TrackEntryFields::ContentEncodings(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum TrackEntryFields {
    TrackNumber(Ebml<u64>),
    TrackUid(Ebml<u64>),
    TrackType(Ebml<u64>),
    FlagEnabled(Ebml<u64>),
    FlagDefault(Ebml<u64>),
    FlagForced(Ebml<u64>),
    FlagHearingImpaired(Ebml<u64>),
    FlagVisualImpaired(Ebml<u64>),
    FlagTextDescriptions(Ebml<u64>),
    FlagOriginal(Ebml<u64>),
    FlagCommentary(Ebml<u64>),
    FlagLacing(Ebml<u64>),
    MinCache(Ebml<u64>),
    MaxCache(Ebml<u64>),
    DefaultDuration(Ebml<u64>),
    DefaultDecodedFieldDuration(Ebml<u64>),
    TrackTimestampScale(Ebml<f64>),
    TrackOffset(Ebml<i64>),
    MaxBlockAdditionId(Ebml<u64>),
    BlockAdditionMapping(Ebml<BlockAdditionMapping>),
    Name(Ebml<String>),
    Language(Ebml<String>),
    LanguageBcp47(Ebml<String>),
    CodecId(Ebml<String>),
    CodecPrivate(Ebml<Vec<u8>>),
    CodecName(Ebml<String>),
    AttachmentLink(Ebml<u64>),
    CodecSettings(Ebml<String>),
    CodecInfoUrl(Ebml<String>),
    CodecDownloadUrl(Ebml<String>),
    CodecDecodeAll(Ebml<u64>),
    TrackOverlay(Ebml<u64>),
    CodecDelay(Ebml<u64>),
    SeekPreRoll(Ebml<u64>),
    TrackTranslate(Ebml<TrackTranslate>),
    Video(Ebml<Video>),
    Audio(Ebml<Audio>),
    TrackOperation(Ebml<TrackOperation>),
    TrickTrackUid(Ebml<u64>),
    TrickTrackSegmentUid(Ebml<Vec<u8>>),
    TrickTrackFlag(Ebml<u64>),
    TrickStructTrackUid(Ebml<u64>),
    TrickStructTrackSegmentUid(Ebml<Vec<u8>>),
    ContentEncodings(Ebml<ContentEncodings>),
}
impl TrackEntryFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::TrackNumber(val) => (EbmlId::TrackNumber, val.index, val.id),
            Self::TrackUid(val) => (EbmlId::TrackUid, val.index, val.id),
            Self::TrackType(val) => (EbmlId::TrackType, val.index, val.id),
            Self::FlagEnabled(val) => (EbmlId::FlagEnabled, val.index, val.id),
            Self::FlagDefault(val) => (EbmlId::FlagDefault, val.index, val.id),
            Self::FlagForced(val) => (EbmlId::FlagForced, val.index, val.id),
            Self::FlagHearingImpaired(val) => (EbmlId::FlagHearingImpaired, val.index, val.id),
            Self::FlagVisualImpaired(val) => (EbmlId::FlagVisualImpaired, val.index, val.id),
            Self::FlagTextDescriptions(val) => (EbmlId::FlagTextDescriptions, val.index, val.id),
            Self::FlagOriginal(val) => (EbmlId::FlagOriginal, val.index, val.id),
            Self::FlagCommentary(val) => (EbmlId::FlagCommentary, val.index, val.id),
            Self::FlagLacing(val) => (EbmlId::FlagLacing, val.index, val.id),
            Self::MinCache(val) => (EbmlId::MinCache, val.index, val.id),
            Self::MaxCache(val) => (EbmlId::MaxCache, val.index, val.id),
            Self::DefaultDuration(val) => (EbmlId::DefaultDuration, val.index, val.id),
            Self::DefaultDecodedFieldDuration(val) => (EbmlId::DefaultDecodedFieldDuration, val.index, val.id),
            Self::TrackTimestampScale(val) => (EbmlId::TrackTimestampScale, val.index, val.id),
            Self::TrackOffset(val) => (EbmlId::TrackOffset, val.index, val.id),
            Self::MaxBlockAdditionId(val) => (EbmlId::MaxBlockAdditionId, val.index, val.id),
            Self::BlockAdditionMapping(val) => (EbmlId::BlockAdditionMapping, val.index, val.id),
            Self::Name(val) => (EbmlId::Name, val.index, val.id),
            Self::Language(val) => (EbmlId::Language, val.index, val.id),
            Self::LanguageBcp47(val) => (EbmlId::LanguageBcp47, val.index, val.id),
            Self::CodecId(val) => (EbmlId::CodecId, val.index, val.id),
            Self::CodecPrivate(val) => (EbmlId::CodecPrivate, val.index, val.id),
            Self::CodecName(val) => (EbmlId::CodecName, val.index, val.id),
            Self::AttachmentLink(val) => (EbmlId::AttachmentLink, val.index, val.id),
            Self::CodecSettings(val) => (EbmlId::CodecSettings, val.index, val.id),
            Self::CodecInfoUrl(val) => (EbmlId::CodecInfoUrl, val.index, val.id),
            Self::CodecDownloadUrl(val) => (EbmlId::CodecDownloadUrl, val.index, val.id),
            Self::CodecDecodeAll(val) => (EbmlId::CodecDecodeAll, val.index, val.id),
            Self::TrackOverlay(val) => (EbmlId::TrackOverlay, val.index, val.id),
            Self::CodecDelay(val) => (EbmlId::CodecDelay, val.index, val.id),
            Self::SeekPreRoll(val) => (EbmlId::SeekPreRoll, val.index, val.id),
            Self::TrackTranslate(val) => (EbmlId::TrackTranslate, val.index, val.id),
            Self::Video(val) => (EbmlId::Video, val.index, val.id),
            Self::Audio(val) => (EbmlId::Audio, val.index, val.id),
            Self::TrackOperation(val) => (EbmlId::TrackOperation, val.index, val.id),
            Self::TrickTrackUid(val) => (EbmlId::TrickTrackUid, val.index, val.id),
            Self::TrickTrackSegmentUid(val) => (EbmlId::TrickTrackSegmentUid, val.index, val.id),
            Self::TrickTrackFlag(val) => (EbmlId::TrickTrackFlag, val.index, val.id),
            Self::TrickStructTrackUid(val) => (EbmlId::TrickStructTrackUid, val.index, val.id),
            Self::TrickStructTrackSegmentUid(val) => (EbmlId::TrickStructTrackSegmentUid, val.index, val.id),
            Self::ContentEncodings(val) => (EbmlId::ContentEncodings, val.index, val.id),
        }
    }
}
crate::impl_ord!(TrackEntryFields);

#[derive(Debug, Clone, Default)]
pub struct BlockAdditionMapping {
    pub size: u64,

    pub block_add_id_value: Option<Ebml<u64>>,
    pub block_add_id_name: Option<Ebml<String>>,
    pub block_add_id_type: Ebml<u64>,
    pub block_add_id_extra_data: Option<Ebml<Vec<u8>>>,
}
impl BlockAdditionMapping {
    pub fn elements(&self) -> std::collections::BTreeSet<BlockAdditionMappingFields> {
        let mut elements = std::collections::BTreeSet::new();
        if let Some(el) = &self.block_add_id_value { elements.insert(BlockAdditionMappingFields::BlockAddIdValue(el.clone())); }
        if let Some(el) = &self.block_add_id_name { elements.insert(BlockAdditionMappingFields::BlockAddIdName(el.clone())); }
        elements.insert(BlockAdditionMappingFields::BlockAddIdType(self.block_add_id_type.clone()));
        if let Some(el) = &self.block_add_id_extra_data { elements.insert(BlockAdditionMappingFields::BlockAddIdExtraData(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum BlockAdditionMappingFields {
    BlockAddIdValue(Ebml<u64>),
    BlockAddIdName(Ebml<String>),
    BlockAddIdType(Ebml<u64>),
    BlockAddIdExtraData(Ebml<Vec<u8>>),
}
impl BlockAdditionMappingFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::BlockAddIdValue(val) => (EbmlId::BlockAddIdValue, val.index, val.id),
            Self::BlockAddIdName(val) => (EbmlId::BlockAddIdName, val.index, val.id),
            Self::BlockAddIdType(val) => (EbmlId::BlockAddIdType, val.index, val.id),
            Self::BlockAddIdExtraData(val) => (EbmlId::BlockAddIdExtraData, val.index, val.id),
        }
    }
}
crate::impl_ord!(BlockAdditionMappingFields);

#[derive(Debug, Clone, Default)]
pub struct TrackTranslate {
    pub size: u64,

    pub track_translate_track_id: Ebml<Vec<u8>>,
    pub track_translate_codec: Ebml<u64>,
    pub track_translate_edition_uid: Vec<Ebml<u64>>,
}
impl TrackTranslate {
    pub fn elements(&self) -> std::collections::BTreeSet<TrackTranslateFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(TrackTranslateFields::TrackTranslateTrackId(self.track_translate_track_id.clone()));
        elements.insert(TrackTranslateFields::TrackTranslateCodec(self.track_translate_codec.clone()));
        for el in &self.track_translate_edition_uid { elements.insert(TrackTranslateFields::TrackTranslateEditionUid(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum TrackTranslateFields {
    TrackTranslateTrackId(Ebml<Vec<u8>>),
    TrackTranslateCodec(Ebml<u64>),
    TrackTranslateEditionUid(Ebml<u64>),
}
impl TrackTranslateFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::TrackTranslateTrackId(val) => (EbmlId::TrackTranslateTrackId, val.index, val.id),
            Self::TrackTranslateCodec(val) => (EbmlId::TrackTranslateCodec, val.index, val.id),
            Self::TrackTranslateEditionUid(val) => (EbmlId::TrackTranslateEditionUid, val.index, val.id),
        }
    }
}
crate::impl_ord!(TrackTranslateFields);

#[derive(Debug, Clone, Default)]
pub struct Video {
    pub size: u64,

    pub flag_interlaced: Ebml<u64>,
    pub field_order: Ebml<u64>,
    pub stereo_mode: Ebml<u64>,
    pub alpha_mode: Ebml<u64>,
    pub old_stereo_mode: Option<Ebml<u64>>,
    pub pixel_width: Ebml<u64>,
    pub pixel_height: Ebml<u64>,
    pub pixel_crop_bottom: Ebml<u64>,
    pub pixel_crop_top: Ebml<u64>,
    pub pixel_crop_left: Ebml<u64>,
    pub pixel_crop_right: Ebml<u64>,
    pub display_width: Option<Ebml<u64>>,
    pub display_height: Option<Ebml<u64>>,
    pub display_unit: Ebml<u64>,
    pub aspect_ratio_type: Option<Ebml<u64>>,
    pub uncompressed_four_cc: Option<Ebml<Vec<u8>>>,
    pub gamma_value: Option<Ebml<f64>>,
    pub frame_rate: Option<Ebml<f64>>,
    pub colour: Option<Ebml<Colour>>,
    pub projection: Option<Ebml<Projection>>,
}
impl Video {
    pub fn elements(&self) -> std::collections::BTreeSet<VideoFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(VideoFields::FlagInterlaced(self.flag_interlaced.clone()));
        elements.insert(VideoFields::FieldOrder(self.field_order.clone()));
        elements.insert(VideoFields::StereoMode(self.stereo_mode.clone()));
        elements.insert(VideoFields::AlphaMode(self.alpha_mode.clone()));
        if let Some(el) = &self.old_stereo_mode { elements.insert(VideoFields::OldStereoMode(el.clone())); }
        elements.insert(VideoFields::PixelWidth(self.pixel_width.clone()));
        elements.insert(VideoFields::PixelHeight(self.pixel_height.clone()));
        elements.insert(VideoFields::PixelCropBottom(self.pixel_crop_bottom.clone()));
        elements.insert(VideoFields::PixelCropTop(self.pixel_crop_top.clone()));
        elements.insert(VideoFields::PixelCropLeft(self.pixel_crop_left.clone()));
        elements.insert(VideoFields::PixelCropRight(self.pixel_crop_right.clone()));
        if let Some(el) = &self.display_width { elements.insert(VideoFields::DisplayWidth(el.clone())); }
        if let Some(el) = &self.display_height { elements.insert(VideoFields::DisplayHeight(el.clone())); }
        elements.insert(VideoFields::DisplayUnit(self.display_unit.clone()));
        if let Some(el) = &self.aspect_ratio_type { elements.insert(VideoFields::AspectRatioType(el.clone())); }
        if let Some(el) = &self.uncompressed_four_cc { elements.insert(VideoFields::UncompressedFourCc(el.clone())); }
        if let Some(el) = &self.gamma_value { elements.insert(VideoFields::GammaValue(el.clone())); }
        if let Some(el) = &self.frame_rate { elements.insert(VideoFields::FrameRate(el.clone())); }
        if let Some(el) = &self.colour { elements.insert(VideoFields::Colour(el.clone())); }
        if let Some(el) = &self.projection { elements.insert(VideoFields::Projection(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum VideoFields {
    FlagInterlaced(Ebml<u64>),
    FieldOrder(Ebml<u64>),
    StereoMode(Ebml<u64>),
    AlphaMode(Ebml<u64>),
    OldStereoMode(Ebml<u64>),
    PixelWidth(Ebml<u64>),
    PixelHeight(Ebml<u64>),
    PixelCropBottom(Ebml<u64>),
    PixelCropTop(Ebml<u64>),
    PixelCropLeft(Ebml<u64>),
    PixelCropRight(Ebml<u64>),
    DisplayWidth(Ebml<u64>),
    DisplayHeight(Ebml<u64>),
    DisplayUnit(Ebml<u64>),
    AspectRatioType(Ebml<u64>),
    UncompressedFourCc(Ebml<Vec<u8>>),
    GammaValue(Ebml<f64>),
    FrameRate(Ebml<f64>),
    Colour(Ebml<Colour>),
    Projection(Ebml<Projection>),
}
impl VideoFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::FlagInterlaced(val) => (EbmlId::FlagInterlaced, val.index, val.id),
            Self::FieldOrder(val) => (EbmlId::FieldOrder, val.index, val.id),
            Self::StereoMode(val) => (EbmlId::StereoMode, val.index, val.id),
            Self::AlphaMode(val) => (EbmlId::AlphaMode, val.index, val.id),
            Self::OldStereoMode(val) => (EbmlId::OldStereoMode, val.index, val.id),
            Self::PixelWidth(val) => (EbmlId::PixelWidth, val.index, val.id),
            Self::PixelHeight(val) => (EbmlId::PixelHeight, val.index, val.id),
            Self::PixelCropBottom(val) => (EbmlId::PixelCropBottom, val.index, val.id),
            Self::PixelCropTop(val) => (EbmlId::PixelCropTop, val.index, val.id),
            Self::PixelCropLeft(val) => (EbmlId::PixelCropLeft, val.index, val.id),
            Self::PixelCropRight(val) => (EbmlId::PixelCropRight, val.index, val.id),
            Self::DisplayWidth(val) => (EbmlId::DisplayWidth, val.index, val.id),
            Self::DisplayHeight(val) => (EbmlId::DisplayHeight, val.index, val.id),
            Self::DisplayUnit(val) => (EbmlId::DisplayUnit, val.index, val.id),
            Self::AspectRatioType(val) => (EbmlId::AspectRatioType, val.index, val.id),
            Self::UncompressedFourCc(val) => (EbmlId::UncompressedFourCc, val.index, val.id),
            Self::GammaValue(val) => (EbmlId::GammaValue, val.index, val.id),
            Self::FrameRate(val) => (EbmlId::FrameRate, val.index, val.id),
            Self::Colour(val) => (EbmlId::Colour, val.index, val.id),
            Self::Projection(val) => (EbmlId::Projection, val.index, val.id),
        }
    }
}
crate::impl_ord!(VideoFields);

#[derive(Debug, Clone, Default)]
pub struct Colour {
    pub size: u64,

    pub matrix_coefficients: Ebml<u64>,
    pub bits_per_channel: Ebml<u64>,
    pub chroma_subsampling_horz: Option<Ebml<u64>>,
    pub chroma_subsampling_vert: Option<Ebml<u64>>,
    pub cb_subsampling_horz: Option<Ebml<u64>>,
    pub cb_subsampling_vert: Option<Ebml<u64>>,
    pub chroma_siting_horz: Ebml<u64>,
    pub chroma_siting_vert: Ebml<u64>,
    pub range: Ebml<u64>,
    pub transfer_characteristics: Ebml<u64>,
    pub primaries: Ebml<u64>,
    pub max_cll: Option<Ebml<u64>>,
    pub max_fall: Option<Ebml<u64>>,
    pub structing_metadata: Option<Ebml<StructingMetadata>>,
}
impl Colour {
    pub fn elements(&self) -> std::collections::BTreeSet<ColourFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ColourFields::MatrixCoefficients(self.matrix_coefficients.clone()));
        elements.insert(ColourFields::BitsPerChannel(self.bits_per_channel.clone()));
        if let Some(el) = &self.chroma_subsampling_horz { elements.insert(ColourFields::ChromaSubsamplingHorz(el.clone())); }
        if let Some(el) = &self.chroma_subsampling_vert { elements.insert(ColourFields::ChromaSubsamplingVert(el.clone())); }
        if let Some(el) = &self.cb_subsampling_horz { elements.insert(ColourFields::CbSubsamplingHorz(el.clone())); }
        if let Some(el) = &self.cb_subsampling_vert { elements.insert(ColourFields::CbSubsamplingVert(el.clone())); }
        elements.insert(ColourFields::ChromaSitingHorz(self.chroma_siting_horz.clone()));
        elements.insert(ColourFields::ChromaSitingVert(self.chroma_siting_vert.clone()));
        elements.insert(ColourFields::Range(self.range.clone()));
        elements.insert(ColourFields::TransferCharacteristics(self.transfer_characteristics.clone()));
        elements.insert(ColourFields::Primaries(self.primaries.clone()));
        if let Some(el) = &self.max_cll { elements.insert(ColourFields::MaxCll(el.clone())); }
        if let Some(el) = &self.max_fall { elements.insert(ColourFields::MaxFall(el.clone())); }
        if let Some(el) = &self.structing_metadata { elements.insert(ColourFields::StructingMetadata(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ColourFields {
    MatrixCoefficients(Ebml<u64>),
    BitsPerChannel(Ebml<u64>),
    ChromaSubsamplingHorz(Ebml<u64>),
    ChromaSubsamplingVert(Ebml<u64>),
    CbSubsamplingHorz(Ebml<u64>),
    CbSubsamplingVert(Ebml<u64>),
    ChromaSitingHorz(Ebml<u64>),
    ChromaSitingVert(Ebml<u64>),
    Range(Ebml<u64>),
    TransferCharacteristics(Ebml<u64>),
    Primaries(Ebml<u64>),
    MaxCll(Ebml<u64>),
    MaxFall(Ebml<u64>),
    StructingMetadata(Ebml<StructingMetadata>),
}
impl ColourFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::MatrixCoefficients(val) => (EbmlId::MatrixCoefficients, val.index, val.id),
            Self::BitsPerChannel(val) => (EbmlId::BitsPerChannel, val.index, val.id),
            Self::ChromaSubsamplingHorz(val) => (EbmlId::ChromaSubsamplingHorz, val.index, val.id),
            Self::ChromaSubsamplingVert(val) => (EbmlId::ChromaSubsamplingVert, val.index, val.id),
            Self::CbSubsamplingHorz(val) => (EbmlId::CbSubsamplingHorz, val.index, val.id),
            Self::CbSubsamplingVert(val) => (EbmlId::CbSubsamplingVert, val.index, val.id),
            Self::ChromaSitingHorz(val) => (EbmlId::ChromaSitingHorz, val.index, val.id),
            Self::ChromaSitingVert(val) => (EbmlId::ChromaSitingVert, val.index, val.id),
            Self::Range(val) => (EbmlId::Range, val.index, val.id),
            Self::TransferCharacteristics(val) => (EbmlId::TransferCharacteristics, val.index, val.id),
            Self::Primaries(val) => (EbmlId::Primaries, val.index, val.id),
            Self::MaxCll(val) => (EbmlId::MaxCll, val.index, val.id),
            Self::MaxFall(val) => (EbmlId::MaxFall, val.index, val.id),
            Self::StructingMetadata(val) => (EbmlId::StructingMetadata, val.index, val.id),
        }
    }
}
crate::impl_ord!(ColourFields);

#[derive(Debug, Clone, Default)]
pub struct StructingMetadata {
    pub size: u64,

    pub primary_r_chromaticity_x: Option<Ebml<f64>>,
    pub primary_r_chromaticity_y: Option<Ebml<f64>>,
    pub primary_g_chromaticity_x: Option<Ebml<f64>>,
    pub primary_g_chromaticity_y: Option<Ebml<f64>>,
    pub primary_b_chromaticity_x: Option<Ebml<f64>>,
    pub primary_b_chromaticity_y: Option<Ebml<f64>>,
    pub white_point_chromaticity_x: Option<Ebml<f64>>,
    pub white_point_chromaticity_y: Option<Ebml<f64>>,
    pub luminance_max: Option<Ebml<f64>>,
    pub luminance_min: Option<Ebml<f64>>,
}
impl StructingMetadata {
    pub fn elements(&self) -> std::collections::BTreeSet<StructingMetadataFields> {
        let mut elements = std::collections::BTreeSet::new();
        if let Some(el) = &self.primary_r_chromaticity_x { elements.insert(StructingMetadataFields::PrimaryRChromaticityX(el.clone())); }
        if let Some(el) = &self.primary_r_chromaticity_y { elements.insert(StructingMetadataFields::PrimaryRChromaticityY(el.clone())); }
        if let Some(el) = &self.primary_g_chromaticity_x { elements.insert(StructingMetadataFields::PrimaryGChromaticityX(el.clone())); }
        if let Some(el) = &self.primary_g_chromaticity_y { elements.insert(StructingMetadataFields::PrimaryGChromaticityY(el.clone())); }
        if let Some(el) = &self.primary_b_chromaticity_x { elements.insert(StructingMetadataFields::PrimaryBChromaticityX(el.clone())); }
        if let Some(el) = &self.primary_b_chromaticity_y { elements.insert(StructingMetadataFields::PrimaryBChromaticityY(el.clone())); }
        if let Some(el) = &self.white_point_chromaticity_x { elements.insert(StructingMetadataFields::WhitePointChromaticityX(el.clone())); }
        if let Some(el) = &self.white_point_chromaticity_y { elements.insert(StructingMetadataFields::WhitePointChromaticityY(el.clone())); }
        if let Some(el) = &self.luminance_max { elements.insert(StructingMetadataFields::LuminanceMax(el.clone())); }
        if let Some(el) = &self.luminance_min { elements.insert(StructingMetadataFields::LuminanceMin(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum StructingMetadataFields {
    PrimaryRChromaticityX(Ebml<f64>),
    PrimaryRChromaticityY(Ebml<f64>),
    PrimaryGChromaticityX(Ebml<f64>),
    PrimaryGChromaticityY(Ebml<f64>),
    PrimaryBChromaticityX(Ebml<f64>),
    PrimaryBChromaticityY(Ebml<f64>),
    WhitePointChromaticityX(Ebml<f64>),
    WhitePointChromaticityY(Ebml<f64>),
    LuminanceMax(Ebml<f64>),
    LuminanceMin(Ebml<f64>),
}
impl StructingMetadataFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::PrimaryRChromaticityX(val) => (EbmlId::PrimaryRChromaticityX, val.index, val.id),
            Self::PrimaryRChromaticityY(val) => (EbmlId::PrimaryRChromaticityY, val.index, val.id),
            Self::PrimaryGChromaticityX(val) => (EbmlId::PrimaryGChromaticityX, val.index, val.id),
            Self::PrimaryGChromaticityY(val) => (EbmlId::PrimaryGChromaticityY, val.index, val.id),
            Self::PrimaryBChromaticityX(val) => (EbmlId::PrimaryBChromaticityX, val.index, val.id),
            Self::PrimaryBChromaticityY(val) => (EbmlId::PrimaryBChromaticityY, val.index, val.id),
            Self::WhitePointChromaticityX(val) => (EbmlId::WhitePointChromaticityX, val.index, val.id),
            Self::WhitePointChromaticityY(val) => (EbmlId::WhitePointChromaticityY, val.index, val.id),
            Self::LuminanceMax(val) => (EbmlId::LuminanceMax, val.index, val.id),
            Self::LuminanceMin(val) => (EbmlId::LuminanceMin, val.index, val.id),
        }
    }
}
crate::impl_ord!(StructingMetadataFields);

#[derive(Debug, Clone, Default)]
pub struct Projection {
    pub size: u64,

    pub projection_type: Ebml<u64>,
    pub projection_private: Option<Ebml<Vec<u8>>>,
    pub projection_pose_yaw: Ebml<f64>,
    pub projection_pose_pitch: Ebml<f64>,
    pub projection_pose_roll: Ebml<f64>,
}
impl Projection {
    pub fn elements(&self) -> std::collections::BTreeSet<ProjectionFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ProjectionFields::ProjectionType(self.projection_type.clone()));
        if let Some(el) = &self.projection_private { elements.insert(ProjectionFields::ProjectionPrivate(el.clone())); }
        elements.insert(ProjectionFields::ProjectionPoseYaw(self.projection_pose_yaw.clone()));
        elements.insert(ProjectionFields::ProjectionPosePitch(self.projection_pose_pitch.clone()));
        elements.insert(ProjectionFields::ProjectionPoseRoll(self.projection_pose_roll.clone()));
        elements
    }
}
#[derive(Debug)]
pub enum ProjectionFields {
    ProjectionType(Ebml<u64>),
    ProjectionPrivate(Ebml<Vec<u8>>),
    ProjectionPoseYaw(Ebml<f64>),
    ProjectionPosePitch(Ebml<f64>),
    ProjectionPoseRoll(Ebml<f64>),
}
impl ProjectionFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ProjectionType(val) => (EbmlId::ProjectionType, val.index, val.id),
            Self::ProjectionPrivate(val) => (EbmlId::ProjectionPrivate, val.index, val.id),
            Self::ProjectionPoseYaw(val) => (EbmlId::ProjectionPoseYaw, val.index, val.id),
            Self::ProjectionPosePitch(val) => (EbmlId::ProjectionPosePitch, val.index, val.id),
            Self::ProjectionPoseRoll(val) => (EbmlId::ProjectionPoseRoll, val.index, val.id),
        }
    }
}
crate::impl_ord!(ProjectionFields);

#[derive(Debug, Clone, Default)]
pub struct Audio {
    pub size: u64,

    pub sampling_frequency: Ebml<f64>,
    pub output_sampling_frequency: Option<Ebml<f64>>,
    pub channels: Ebml<u64>,
    pub channel_positions: Option<Ebml<Vec<u8>>>,
    pub bit_depth: Option<Ebml<u64>>,
    pub emphasis: Ebml<u64>,
}
impl Audio {
    pub fn elements(&self) -> std::collections::BTreeSet<AudioFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(AudioFields::SamplingFrequency(self.sampling_frequency.clone()));
        if let Some(el) = &self.output_sampling_frequency { elements.insert(AudioFields::OutputSamplingFrequency(el.clone())); }
        elements.insert(AudioFields::Channels(self.channels.clone()));
        if let Some(el) = &self.channel_positions { elements.insert(AudioFields::ChannelPositions(el.clone())); }
        if let Some(el) = &self.bit_depth { elements.insert(AudioFields::BitDepth(el.clone())); }
        elements.insert(AudioFields::Emphasis(self.emphasis.clone()));
        elements
    }
}
#[derive(Debug)]
pub enum AudioFields {
    SamplingFrequency(Ebml<f64>),
    OutputSamplingFrequency(Ebml<f64>),
    Channels(Ebml<u64>),
    ChannelPositions(Ebml<Vec<u8>>),
    BitDepth(Ebml<u64>),
    Emphasis(Ebml<u64>),
}
impl AudioFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::SamplingFrequency(val) => (EbmlId::SamplingFrequency, val.index, val.id),
            Self::OutputSamplingFrequency(val) => (EbmlId::OutputSamplingFrequency, val.index, val.id),
            Self::Channels(val) => (EbmlId::Channels, val.index, val.id),
            Self::ChannelPositions(val) => (EbmlId::ChannelPositions, val.index, val.id),
            Self::BitDepth(val) => (EbmlId::BitDepth, val.index, val.id),
            Self::Emphasis(val) => (EbmlId::Emphasis, val.index, val.id),
        }
    }
}
crate::impl_ord!(AudioFields);

#[derive(Debug, Clone, Default)]
pub struct TrackOperation {
    pub size: u64,

    pub track_combine_planes: Option<Ebml<TrackCombinePlanes>>,
    pub track_join_blocks: Option<Ebml<TrackJoinBlocks>>,
}
impl TrackOperation {
    pub fn elements(&self) -> std::collections::BTreeSet<TrackOperationFields> {
        let mut elements = std::collections::BTreeSet::new();
        if let Some(el) = &self.track_combine_planes { elements.insert(TrackOperationFields::TrackCombinePlanes(el.clone())); }
        if let Some(el) = &self.track_join_blocks { elements.insert(TrackOperationFields::TrackJoinBlocks(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum TrackOperationFields {
    TrackCombinePlanes(Ebml<TrackCombinePlanes>),
    TrackJoinBlocks(Ebml<TrackJoinBlocks>),
}
impl TrackOperationFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::TrackCombinePlanes(val) => (EbmlId::TrackCombinePlanes, val.index, val.id),
            Self::TrackJoinBlocks(val) => (EbmlId::TrackJoinBlocks, val.index, val.id),
        }
    }
}
crate::impl_ord!(TrackOperationFields);

#[derive(Debug, Clone, Default)]
pub struct TrackCombinePlanes {
    pub size: u64,

    pub track_plane: Vec<Ebml<TrackPlane>>,
}
impl TrackCombinePlanes {
    pub fn elements(&self) -> std::collections::BTreeSet<TrackCombinePlanesFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.track_plane { elements.insert(TrackCombinePlanesFields::TrackPlane(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum TrackCombinePlanesFields {
    TrackPlane(Ebml<TrackPlane>),
}
impl TrackCombinePlanesFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::TrackPlane(val) => (EbmlId::TrackPlane, val.index, val.id),
        }
    }
}
crate::impl_ord!(TrackCombinePlanesFields);

#[derive(Debug, Clone, Default)]
pub struct TrackPlane {
    pub size: u64,

    pub track_plane_uid: Ebml<u64>,
    pub track_plane_type: Ebml<u64>,
}
impl TrackPlane {
    pub fn elements(&self) -> std::collections::BTreeSet<TrackPlaneFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(TrackPlaneFields::TrackPlaneUid(self.track_plane_uid.clone()));
        elements.insert(TrackPlaneFields::TrackPlaneType(self.track_plane_type.clone()));
        elements
    }
}
#[derive(Debug)]
pub enum TrackPlaneFields {
    TrackPlaneUid(Ebml<u64>),
    TrackPlaneType(Ebml<u64>),
}
impl TrackPlaneFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::TrackPlaneUid(val) => (EbmlId::TrackPlaneUid, val.index, val.id),
            Self::TrackPlaneType(val) => (EbmlId::TrackPlaneType, val.index, val.id),
        }
    }
}
crate::impl_ord!(TrackPlaneFields);

#[derive(Debug, Clone, Default)]
pub struct TrackJoinBlocks {
    pub size: u64,

    pub track_join_uid: Vec<Ebml<u64>>,
}
impl TrackJoinBlocks {
    pub fn elements(&self) -> std::collections::BTreeSet<TrackJoinBlocksFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.track_join_uid { elements.insert(TrackJoinBlocksFields::TrackJoinUid(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum TrackJoinBlocksFields {
    TrackJoinUid(Ebml<u64>),
}
impl TrackJoinBlocksFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::TrackJoinUid(val) => (EbmlId::TrackJoinUid, val.index, val.id),
        }
    }
}
crate::impl_ord!(TrackJoinBlocksFields);

#[derive(Debug, Clone, Default)]
pub struct ContentEncodings {
    pub size: u64,

    pub content_encoding: Vec<Ebml<ContentEncoding>>,
}
impl ContentEncodings {
    pub fn elements(&self) -> std::collections::BTreeSet<ContentEncodingsFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.content_encoding { elements.insert(ContentEncodingsFields::ContentEncoding(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ContentEncodingsFields {
    ContentEncoding(Ebml<ContentEncoding>),
}
impl ContentEncodingsFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ContentEncoding(val) => (EbmlId::ContentEncoding, val.index, val.id),
        }
    }
}
crate::impl_ord!(ContentEncodingsFields);

#[derive(Debug, Clone, Default)]
pub struct ContentEncoding {
    pub size: u64,

    pub content_encoding_order: Ebml<u64>,
    pub content_encoding_scope: Ebml<u64>,
    pub content_encoding_type: Ebml<u64>,
    pub content_compression: Option<Ebml<ContentCompression>>,
    pub content_encryption: Option<Ebml<ContentEncryption>>,
}
impl ContentEncoding {
    pub fn elements(&self) -> std::collections::BTreeSet<ContentEncodingFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ContentEncodingFields::ContentEncodingOrder(self.content_encoding_order.clone()));
        elements.insert(ContentEncodingFields::ContentEncodingScope(self.content_encoding_scope.clone()));
        elements.insert(ContentEncodingFields::ContentEncodingType(self.content_encoding_type.clone()));
        if let Some(el) = &self.content_compression { elements.insert(ContentEncodingFields::ContentCompression(el.clone())); }
        if let Some(el) = &self.content_encryption { elements.insert(ContentEncodingFields::ContentEncryption(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ContentEncodingFields {
    ContentEncodingOrder(Ebml<u64>),
    ContentEncodingScope(Ebml<u64>),
    ContentEncodingType(Ebml<u64>),
    ContentCompression(Ebml<ContentCompression>),
    ContentEncryption(Ebml<ContentEncryption>),
}
impl ContentEncodingFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ContentEncodingOrder(val) => (EbmlId::ContentEncodingOrder, val.index, val.id),
            Self::ContentEncodingScope(val) => (EbmlId::ContentEncodingScope, val.index, val.id),
            Self::ContentEncodingType(val) => (EbmlId::ContentEncodingType, val.index, val.id),
            Self::ContentCompression(val) => (EbmlId::ContentCompression, val.index, val.id),
            Self::ContentEncryption(val) => (EbmlId::ContentEncryption, val.index, val.id),
        }
    }
}
crate::impl_ord!(ContentEncodingFields);

#[derive(Debug, Clone, Default)]
pub struct ContentCompression {
    pub size: u64,

    pub content_comp_algo: Ebml<u64>,
    pub content_comp_settings: Option<Ebml<Vec<u8>>>,
}
impl ContentCompression {
    pub fn elements(&self) -> std::collections::BTreeSet<ContentCompressionFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ContentCompressionFields::ContentCompAlgo(self.content_comp_algo.clone()));
        if let Some(el) = &self.content_comp_settings { elements.insert(ContentCompressionFields::ContentCompSettings(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ContentCompressionFields {
    ContentCompAlgo(Ebml<u64>),
    ContentCompSettings(Ebml<Vec<u8>>),
}
impl ContentCompressionFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ContentCompAlgo(val) => (EbmlId::ContentCompAlgo, val.index, val.id),
            Self::ContentCompSettings(val) => (EbmlId::ContentCompSettings, val.index, val.id),
        }
    }
}
crate::impl_ord!(ContentCompressionFields);

#[derive(Debug, Clone, Default)]
pub struct ContentEncryption {
    pub size: u64,

    pub content_enc_algo: Ebml<u64>,
    pub content_enc_key_id: Option<Ebml<Vec<u8>>>,
    pub content_enc_aes_settings: Option<Ebml<ContentEncAesSettings>>,
    pub content_signature: Option<Ebml<Vec<u8>>>,
    pub content_sig_key_id: Option<Ebml<Vec<u8>>>,
    pub content_sig_algo: Option<Ebml<u64>>,
    pub content_sig_hash_algo: Option<Ebml<u64>>,
}
impl ContentEncryption {
    pub fn elements(&self) -> std::collections::BTreeSet<ContentEncryptionFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ContentEncryptionFields::ContentEncAlgo(self.content_enc_algo.clone()));
        if let Some(el) = &self.content_enc_key_id { elements.insert(ContentEncryptionFields::ContentEncKeyId(el.clone())); }
        if let Some(el) = &self.content_enc_aes_settings { elements.insert(ContentEncryptionFields::ContentEncAesSettings(el.clone())); }
        if let Some(el) = &self.content_signature { elements.insert(ContentEncryptionFields::ContentSignature(el.clone())); }
        if let Some(el) = &self.content_sig_key_id { elements.insert(ContentEncryptionFields::ContentSigKeyId(el.clone())); }
        if let Some(el) = &self.content_sig_algo { elements.insert(ContentEncryptionFields::ContentSigAlgo(el.clone())); }
        if let Some(el) = &self.content_sig_hash_algo { elements.insert(ContentEncryptionFields::ContentSigHashAlgo(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ContentEncryptionFields {
    ContentEncAlgo(Ebml<u64>),
    ContentEncKeyId(Ebml<Vec<u8>>),
    ContentEncAesSettings(Ebml<ContentEncAesSettings>),
    ContentSignature(Ebml<Vec<u8>>),
    ContentSigKeyId(Ebml<Vec<u8>>),
    ContentSigAlgo(Ebml<u64>),
    ContentSigHashAlgo(Ebml<u64>),
}
impl ContentEncryptionFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ContentEncAlgo(val) => (EbmlId::ContentEncAlgo, val.index, val.id),
            Self::ContentEncKeyId(val) => (EbmlId::ContentEncKeyId, val.index, val.id),
            Self::ContentEncAesSettings(val) => (EbmlId::ContentEncAesSettings, val.index, val.id),
            Self::ContentSignature(val) => (EbmlId::ContentSignature, val.index, val.id),
            Self::ContentSigKeyId(val) => (EbmlId::ContentSigKeyId, val.index, val.id),
            Self::ContentSigAlgo(val) => (EbmlId::ContentSigAlgo, val.index, val.id),
            Self::ContentSigHashAlgo(val) => (EbmlId::ContentSigHashAlgo, val.index, val.id),
        }
    }
}
crate::impl_ord!(ContentEncryptionFields);

#[derive(Debug, Clone, Default)]
pub struct ContentEncAesSettings {
    pub size: u64,

    pub aes_settings_cipher_mode: Ebml<u64>,
}
impl ContentEncAesSettings {
    pub fn elements(&self) -> std::collections::BTreeSet<ContentEncAesSettingsFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ContentEncAesSettingsFields::AesSettingsCipherMode(self.aes_settings_cipher_mode.clone()));
        elements
    }
}
#[derive(Debug)]
pub enum ContentEncAesSettingsFields {
    AesSettingsCipherMode(Ebml<u64>),
}
impl ContentEncAesSettingsFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::AesSettingsCipherMode(val) => (EbmlId::AesSettingsCipherMode, val.index, val.id),
        }
    }
}
crate::impl_ord!(ContentEncAesSettingsFields);

#[derive(Debug, Clone, Default)]
pub struct Cues {
    pub size: u64,

    pub cue_point: Vec<Ebml<CuePoint>>,
}
impl Cues {
    pub fn elements(&self) -> std::collections::BTreeSet<CuesFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.cue_point { elements.insert(CuesFields::CuePoint(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum CuesFields {
    CuePoint(Ebml<CuePoint>),
}
impl CuesFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::CuePoint(val) => (EbmlId::CuePoint, val.index, val.id),
        }
    }
}
crate::impl_ord!(CuesFields);

#[derive(Debug, Clone, Default)]
pub struct CuePoint {
    pub size: u64,

    pub cue_time: Ebml<u64>,
    pub cue_track_positions: Vec<Ebml<CueTrackPositions>>,
}
impl CuePoint {
    pub fn elements(&self) -> std::collections::BTreeSet<CuePointFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(CuePointFields::CueTime(self.cue_time.clone()));
        for el in &self.cue_track_positions { elements.insert(CuePointFields::CueTrackPositions(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum CuePointFields {
    CueTime(Ebml<u64>),
    CueTrackPositions(Ebml<CueTrackPositions>),
}
impl CuePointFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::CueTime(val) => (EbmlId::CueTime, val.index, val.id),
            Self::CueTrackPositions(val) => (EbmlId::CueTrackPositions, val.index, val.id),
        }
    }
}
crate::impl_ord!(CuePointFields);

#[derive(Debug, Clone, Default)]
pub struct CueTrackPositions {
    pub size: u64,

    pub cue_track: Ebml<u64>,
    pub cue_cluster_position: Ebml<u64>,
    pub cue_relative_position: Option<Ebml<u64>>,
    pub cue_duration: Option<Ebml<u64>>,
    pub cue_block_number: Option<Ebml<u64>>,
    pub cue_codec_state: Ebml<u64>,
    pub cue_reference: Vec<Ebml<CueReference>>,
}
impl CueTrackPositions {
    pub fn elements(&self) -> std::collections::BTreeSet<CueTrackPositionsFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(CueTrackPositionsFields::CueTrack(self.cue_track.clone()));
        elements.insert(CueTrackPositionsFields::CueClusterPosition(self.cue_cluster_position.clone()));
        if let Some(el) = &self.cue_relative_position { elements.insert(CueTrackPositionsFields::CueRelativePosition(el.clone())); }
        if let Some(el) = &self.cue_duration { elements.insert(CueTrackPositionsFields::CueDuration(el.clone())); }
        if let Some(el) = &self.cue_block_number { elements.insert(CueTrackPositionsFields::CueBlockNumber(el.clone())); }
        elements.insert(CueTrackPositionsFields::CueCodecState(self.cue_codec_state.clone()));
        for el in &self.cue_reference { elements.insert(CueTrackPositionsFields::CueReference(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum CueTrackPositionsFields {
    CueTrack(Ebml<u64>),
    CueClusterPosition(Ebml<u64>),
    CueRelativePosition(Ebml<u64>),
    CueDuration(Ebml<u64>),
    CueBlockNumber(Ebml<u64>),
    CueCodecState(Ebml<u64>),
    CueReference(Ebml<CueReference>),
}
impl CueTrackPositionsFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::CueTrack(val) => (EbmlId::CueTrack, val.index, val.id),
            Self::CueClusterPosition(val) => (EbmlId::CueClusterPosition, val.index, val.id),
            Self::CueRelativePosition(val) => (EbmlId::CueRelativePosition, val.index, val.id),
            Self::CueDuration(val) => (EbmlId::CueDuration, val.index, val.id),
            Self::CueBlockNumber(val) => (EbmlId::CueBlockNumber, val.index, val.id),
            Self::CueCodecState(val) => (EbmlId::CueCodecState, val.index, val.id),
            Self::CueReference(val) => (EbmlId::CueReference, val.index, val.id),
        }
    }
}
crate::impl_ord!(CueTrackPositionsFields);

#[derive(Debug, Clone, Default)]
pub struct CueReference {
    pub size: u64,

    pub cue_ref_time: Ebml<u64>,
    pub cue_ref_cluster: Ebml<u64>,
    pub cue_ref_number: Option<Ebml<u64>>,
    pub cue_ref_codec_state: Option<Ebml<u64>>,
}
impl CueReference {
    pub fn elements(&self) -> std::collections::BTreeSet<CueReferenceFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(CueReferenceFields::CueRefTime(self.cue_ref_time.clone()));
        elements.insert(CueReferenceFields::CueRefCluster(self.cue_ref_cluster.clone()));
        if let Some(el) = &self.cue_ref_number { elements.insert(CueReferenceFields::CueRefNumber(el.clone())); }
        if let Some(el) = &self.cue_ref_codec_state { elements.insert(CueReferenceFields::CueRefCodecState(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum CueReferenceFields {
    CueRefTime(Ebml<u64>),
    CueRefCluster(Ebml<u64>),
    CueRefNumber(Ebml<u64>),
    CueRefCodecState(Ebml<u64>),
}
impl CueReferenceFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::CueRefTime(val) => (EbmlId::CueRefTime, val.index, val.id),
            Self::CueRefCluster(val) => (EbmlId::CueRefCluster, val.index, val.id),
            Self::CueRefNumber(val) => (EbmlId::CueRefNumber, val.index, val.id),
            Self::CueRefCodecState(val) => (EbmlId::CueRefCodecState, val.index, val.id),
        }
    }
}
crate::impl_ord!(CueReferenceFields);

#[derive(Debug, Clone, Default)]
pub struct Attachments {
    pub size: u64,

    pub attached_file: Vec<Ebml<AttachedFile>>,
}
impl Attachments {
    pub fn elements(&self) -> std::collections::BTreeSet<AttachmentsFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.attached_file { elements.insert(AttachmentsFields::AttachedFile(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum AttachmentsFields {
    AttachedFile(Ebml<AttachedFile>),
}
impl AttachmentsFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::AttachedFile(val) => (EbmlId::AttachedFile, val.index, val.id),
        }
    }
}
crate::impl_ord!(AttachmentsFields);

#[derive(Debug, Clone, Default)]
pub struct AttachedFile {
    pub size: u64,

    pub file_description: Option<Ebml<String>>,
    pub file_name: Ebml<String>,
    pub file_media_type: Ebml<String>,
    pub file_data: Ebml<Vec<u8>>,
    pub file_uid: Ebml<u64>,
    pub file_referral: Option<Ebml<Vec<u8>>>,
    pub file_used_start_time: Option<Ebml<u64>>,
    pub file_used_end_time: Option<Ebml<u64>>,
}
impl AttachedFile {
    pub fn elements(&self) -> std::collections::BTreeSet<AttachedFileFields> {
        let mut elements = std::collections::BTreeSet::new();
        if let Some(el) = &self.file_description { elements.insert(AttachedFileFields::FileDescription(el.clone())); }
        elements.insert(AttachedFileFields::FileName(self.file_name.clone()));
        elements.insert(AttachedFileFields::FileMediaType(self.file_media_type.clone()));
        elements.insert(AttachedFileFields::FileData(self.file_data.clone()));
        elements.insert(AttachedFileFields::FileUid(self.file_uid.clone()));
        if let Some(el) = &self.file_referral { elements.insert(AttachedFileFields::FileReferral(el.clone())); }
        if let Some(el) = &self.file_used_start_time { elements.insert(AttachedFileFields::FileUsedStartTime(el.clone())); }
        if let Some(el) = &self.file_used_end_time { elements.insert(AttachedFileFields::FileUsedEndTime(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum AttachedFileFields {
    FileDescription(Ebml<String>),
    FileName(Ebml<String>),
    FileMediaType(Ebml<String>),
    FileData(Ebml<Vec<u8>>),
    FileUid(Ebml<u64>),
    FileReferral(Ebml<Vec<u8>>),
    FileUsedStartTime(Ebml<u64>),
    FileUsedEndTime(Ebml<u64>),
}
impl AttachedFileFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::FileDescription(val) => (EbmlId::FileDescription, val.index, val.id),
            Self::FileName(val) => (EbmlId::FileName, val.index, val.id),
            Self::FileMediaType(val) => (EbmlId::FileMediaType, val.index, val.id),
            Self::FileData(val) => (EbmlId::FileData, val.index, val.id),
            Self::FileUid(val) => (EbmlId::FileUid, val.index, val.id),
            Self::FileReferral(val) => (EbmlId::FileReferral, val.index, val.id),
            Self::FileUsedStartTime(val) => (EbmlId::FileUsedStartTime, val.index, val.id),
            Self::FileUsedEndTime(val) => (EbmlId::FileUsedEndTime, val.index, val.id),
        }
    }
}
crate::impl_ord!(AttachedFileFields);

#[derive(Debug, Clone, Default)]
pub struct Chapters {
    pub size: u64,

    pub edition_entry: Vec<Ebml<EditionEntry>>,
}
impl Chapters {
    pub fn elements(&self) -> std::collections::BTreeSet<ChaptersFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.edition_entry { elements.insert(ChaptersFields::EditionEntry(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ChaptersFields {
    EditionEntry(Ebml<EditionEntry>),
}
impl ChaptersFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::EditionEntry(val) => (EbmlId::EditionEntry, val.index, val.id),
        }
    }
}
crate::impl_ord!(ChaptersFields);

#[derive(Debug, Clone, Default)]
pub struct EditionEntry {
    pub size: u64,

    pub edition_uid: Option<Ebml<u64>>,
    pub edition_flag_hidden: Ebml<u64>,
    pub edition_flag_default: Ebml<u64>,
    pub edition_flag_ordered: Ebml<u64>,
    pub edition_display: Vec<Ebml<EditionDisplay>>,
    pub chapter_atom: Vec<Ebml<ChapterAtom>>,
}
impl EditionEntry {
    pub fn elements(&self) -> std::collections::BTreeSet<EditionEntryFields> {
        let mut elements = std::collections::BTreeSet::new();
        if let Some(el) = &self.edition_uid { elements.insert(EditionEntryFields::EditionUid(el.clone())); }
        elements.insert(EditionEntryFields::EditionFlagHidden(self.edition_flag_hidden.clone()));
        elements.insert(EditionEntryFields::EditionFlagDefault(self.edition_flag_default.clone()));
        elements.insert(EditionEntryFields::EditionFlagOrdered(self.edition_flag_ordered.clone()));
        for el in &self.edition_display { elements.insert(EditionEntryFields::EditionDisplay(el.clone())); }
        for el in &self.chapter_atom { elements.insert(EditionEntryFields::ChapterAtom(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum EditionEntryFields {
    EditionUid(Ebml<u64>),
    EditionFlagHidden(Ebml<u64>),
    EditionFlagDefault(Ebml<u64>),
    EditionFlagOrdered(Ebml<u64>),
    EditionDisplay(Ebml<EditionDisplay>),
    ChapterAtom(Ebml<ChapterAtom>),
}
impl EditionEntryFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::EditionUid(val) => (EbmlId::EditionUid, val.index, val.id),
            Self::EditionFlagHidden(val) => (EbmlId::EditionFlagHidden, val.index, val.id),
            Self::EditionFlagDefault(val) => (EbmlId::EditionFlagDefault, val.index, val.id),
            Self::EditionFlagOrdered(val) => (EbmlId::EditionFlagOrdered, val.index, val.id),
            Self::EditionDisplay(val) => (EbmlId::EditionDisplay, val.index, val.id),
            Self::ChapterAtom(val) => (EbmlId::ChapterAtom, val.index, val.id),
        }
    }
}
crate::impl_ord!(EditionEntryFields);

#[derive(Debug, Clone, Default)]
pub struct EditionDisplay {
    pub size: u64,

    pub edition_string: Ebml<String>,
    pub edition_language_ietf: Vec<Ebml<String>>,
}
impl EditionDisplay {
    pub fn elements(&self) -> std::collections::BTreeSet<EditionDisplayFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(EditionDisplayFields::EditionString(self.edition_string.clone()));
        for el in &self.edition_language_ietf { elements.insert(EditionDisplayFields::EditionLanguageIetf(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum EditionDisplayFields {
    EditionString(Ebml<String>),
    EditionLanguageIetf(Ebml<String>),
}
impl EditionDisplayFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::EditionString(val) => (EbmlId::EditionString, val.index, val.id),
            Self::EditionLanguageIetf(val) => (EbmlId::EditionLanguageIetf, val.index, val.id),
        }
    }
}
crate::impl_ord!(EditionDisplayFields);

#[derive(Debug, Clone, Default)]
pub struct ChapterAtom {
    pub size: u64,

    pub chapter_uid: Ebml<u64>,
    pub chapter_string_uid: Option<Ebml<String>>,
    pub chapter_time_start: Ebml<u64>,
    pub chapter_time_end: Option<Ebml<u64>>,
    pub chapter_flag_hidden: Ebml<u64>,
    pub chapter_flag_enabled: Ebml<u64>,
    pub chapter_segment_uuid: Option<Ebml<Vec<u8>>>,
    pub chapter_skip_type: Option<Ebml<u64>>,
    pub chapter_segment_edition_uid: Option<Ebml<u64>>,
    pub chapter_physical_equiv: Option<Ebml<u64>>,
    pub chapter_track: Option<Ebml<ChapterTrack>>,
    pub chapter_display: Vec<Ebml<ChapterDisplay>>,
    pub chap_process: Vec<Ebml<ChapProcess>>,
}
impl ChapterAtom {
    pub fn elements(&self) -> std::collections::BTreeSet<ChapterAtomFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ChapterAtomFields::ChapterUid(self.chapter_uid.clone()));
        if let Some(el) = &self.chapter_string_uid { elements.insert(ChapterAtomFields::ChapterStringUid(el.clone())); }
        elements.insert(ChapterAtomFields::ChapterTimeStart(self.chapter_time_start.clone()));
        if let Some(el) = &self.chapter_time_end { elements.insert(ChapterAtomFields::ChapterTimeEnd(el.clone())); }
        elements.insert(ChapterAtomFields::ChapterFlagHidden(self.chapter_flag_hidden.clone()));
        elements.insert(ChapterAtomFields::ChapterFlagEnabled(self.chapter_flag_enabled.clone()));
        if let Some(el) = &self.chapter_segment_uuid { elements.insert(ChapterAtomFields::ChapterSegmentUuid(el.clone())); }
        if let Some(el) = &self.chapter_skip_type { elements.insert(ChapterAtomFields::ChapterSkipType(el.clone())); }
        if let Some(el) = &self.chapter_segment_edition_uid { elements.insert(ChapterAtomFields::ChapterSegmentEditionUid(el.clone())); }
        if let Some(el) = &self.chapter_physical_equiv { elements.insert(ChapterAtomFields::ChapterPhysicalEquiv(el.clone())); }
        if let Some(el) = &self.chapter_track { elements.insert(ChapterAtomFields::ChapterTrack(el.clone())); }
        for el in &self.chapter_display { elements.insert(ChapterAtomFields::ChapterDisplay(el.clone())); }
        for el in &self.chap_process { elements.insert(ChapterAtomFields::ChapProcess(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ChapterAtomFields {
    ChapterUid(Ebml<u64>),
    ChapterStringUid(Ebml<String>),
    ChapterTimeStart(Ebml<u64>),
    ChapterTimeEnd(Ebml<u64>),
    ChapterFlagHidden(Ebml<u64>),
    ChapterFlagEnabled(Ebml<u64>),
    ChapterSegmentUuid(Ebml<Vec<u8>>),
    ChapterSkipType(Ebml<u64>),
    ChapterSegmentEditionUid(Ebml<u64>),
    ChapterPhysicalEquiv(Ebml<u64>),
    ChapterTrack(Ebml<ChapterTrack>),
    ChapterDisplay(Ebml<ChapterDisplay>),
    ChapProcess(Ebml<ChapProcess>),
}
impl ChapterAtomFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ChapterUid(val) => (EbmlId::ChapterUid, val.index, val.id),
            Self::ChapterStringUid(val) => (EbmlId::ChapterStringUid, val.index, val.id),
            Self::ChapterTimeStart(val) => (EbmlId::ChapterTimeStart, val.index, val.id),
            Self::ChapterTimeEnd(val) => (EbmlId::ChapterTimeEnd, val.index, val.id),
            Self::ChapterFlagHidden(val) => (EbmlId::ChapterFlagHidden, val.index, val.id),
            Self::ChapterFlagEnabled(val) => (EbmlId::ChapterFlagEnabled, val.index, val.id),
            Self::ChapterSegmentUuid(val) => (EbmlId::ChapterSegmentUuid, val.index, val.id),
            Self::ChapterSkipType(val) => (EbmlId::ChapterSkipType, val.index, val.id),
            Self::ChapterSegmentEditionUid(val) => (EbmlId::ChapterSegmentEditionUid, val.index, val.id),
            Self::ChapterPhysicalEquiv(val) => (EbmlId::ChapterPhysicalEquiv, val.index, val.id),
            Self::ChapterTrack(val) => (EbmlId::ChapterTrack, val.index, val.id),
            Self::ChapterDisplay(val) => (EbmlId::ChapterDisplay, val.index, val.id),
            Self::ChapProcess(val) => (EbmlId::ChapProcess, val.index, val.id),
        }
    }
}
crate::impl_ord!(ChapterAtomFields);

#[derive(Debug, Clone, Default)]
pub struct ChapterTrack {
    pub size: u64,

    pub chapter_track_uid: Vec<Ebml<u64>>,
}
impl ChapterTrack {
    pub fn elements(&self) -> std::collections::BTreeSet<ChapterTrackFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.chapter_track_uid { elements.insert(ChapterTrackFields::ChapterTrackUid(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ChapterTrackFields {
    ChapterTrackUid(Ebml<u64>),
}
impl ChapterTrackFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ChapterTrackUid(val) => (EbmlId::ChapterTrackUid, val.index, val.id),
        }
    }
}
crate::impl_ord!(ChapterTrackFields);

#[derive(Debug, Clone, Default)]
pub struct ChapterDisplay {
    pub size: u64,

    pub chap_string: Ebml<String>,
    pub chap_language: Vec<Ebml<String>>,
    pub chap_language_bcp_47: Vec<Ebml<String>>,
    pub chap_country: Vec<Ebml<String>>,
}
impl ChapterDisplay {
    pub fn elements(&self) -> std::collections::BTreeSet<ChapterDisplayFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ChapterDisplayFields::ChapString(self.chap_string.clone()));
        for el in &self.chap_language { elements.insert(ChapterDisplayFields::ChapLanguage(el.clone())); }
        for el in &self.chap_language_bcp_47 { elements.insert(ChapterDisplayFields::ChapLanguageBcp47(el.clone())); }
        for el in &self.chap_country { elements.insert(ChapterDisplayFields::ChapCountry(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ChapterDisplayFields {
    ChapString(Ebml<String>),
    ChapLanguage(Ebml<String>),
    ChapLanguageBcp47(Ebml<String>),
    ChapCountry(Ebml<String>),
}
impl ChapterDisplayFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ChapString(val) => (EbmlId::ChapString, val.index, val.id),
            Self::ChapLanguage(val) => (EbmlId::ChapLanguage, val.index, val.id),
            Self::ChapLanguageBcp47(val) => (EbmlId::ChapLanguageBcp47, val.index, val.id),
            Self::ChapCountry(val) => (EbmlId::ChapCountry, val.index, val.id),
        }
    }
}
crate::impl_ord!(ChapterDisplayFields);

#[derive(Debug, Clone, Default)]
pub struct ChapProcess {
    pub size: u64,

    pub chap_process_codec_id: Ebml<u64>,
    pub chap_process_private: Option<Ebml<Vec<u8>>>,
    pub chap_process_command: Vec<Ebml<ChapProcessCommand>>,
}
impl ChapProcess {
    pub fn elements(&self) -> std::collections::BTreeSet<ChapProcessFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ChapProcessFields::ChapProcessCodecId(self.chap_process_codec_id.clone()));
        if let Some(el) = &self.chap_process_private { elements.insert(ChapProcessFields::ChapProcessPrivate(el.clone())); }
        for el in &self.chap_process_command { elements.insert(ChapProcessFields::ChapProcessCommand(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum ChapProcessFields {
    ChapProcessCodecId(Ebml<u64>),
    ChapProcessPrivate(Ebml<Vec<u8>>),
    ChapProcessCommand(Ebml<ChapProcessCommand>),
}
impl ChapProcessFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ChapProcessCodecId(val) => (EbmlId::ChapProcessCodecId, val.index, val.id),
            Self::ChapProcessPrivate(val) => (EbmlId::ChapProcessPrivate, val.index, val.id),
            Self::ChapProcessCommand(val) => (EbmlId::ChapProcessCommand, val.index, val.id),
        }
    }
}
crate::impl_ord!(ChapProcessFields);

#[derive(Debug, Clone, Default)]
pub struct ChapProcessCommand {
    pub size: u64,

    pub chap_process_time: Ebml<u64>,
    pub chap_process_data: Ebml<Vec<u8>>,
}
impl ChapProcessCommand {
    pub fn elements(&self) -> std::collections::BTreeSet<ChapProcessCommandFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(ChapProcessCommandFields::ChapProcessTime(self.chap_process_time.clone()));
        elements.insert(ChapProcessCommandFields::ChapProcessData(self.chap_process_data.clone()));
        elements
    }
}
#[derive(Debug)]
pub enum ChapProcessCommandFields {
    ChapProcessTime(Ebml<u64>),
    ChapProcessData(Ebml<Vec<u8>>),
}
impl ChapProcessCommandFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::ChapProcessTime(val) => (EbmlId::ChapProcessTime, val.index, val.id),
            Self::ChapProcessData(val) => (EbmlId::ChapProcessData, val.index, val.id),
        }
    }
}
crate::impl_ord!(ChapProcessCommandFields);

#[derive(Debug, Clone, Default)]
pub struct Tags {
    pub size: u64,

    pub tag: Vec<Ebml<Tag>>,
}
impl Tags {
    pub fn elements(&self) -> std::collections::BTreeSet<TagsFields> {
        let mut elements = std::collections::BTreeSet::new();
        for el in &self.tag { elements.insert(TagsFields::Tag(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum TagsFields {
    Tag(Ebml<Tag>),
}
impl TagsFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::Tag(val) => (EbmlId::Tag, val.index, val.id),
        }
    }
}
crate::impl_ord!(TagsFields);

#[derive(Debug, Clone, Default)]
pub struct Tag {
    pub size: u64,

    pub targets: Ebml<Targets>,
    pub simple_tag: Vec<Ebml<SimpleTag>>,
}
impl Tag {
    pub fn elements(&self) -> std::collections::BTreeSet<TagFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(TagFields::Targets(self.targets.clone()));
        for el in &self.simple_tag { elements.insert(TagFields::SimpleTag(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum TagFields {
    Targets(Ebml<Targets>),
    SimpleTag(Ebml<SimpleTag>),
}
impl TagFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::Targets(val) => (EbmlId::Targets, val.index, val.id),
            Self::SimpleTag(val) => (EbmlId::SimpleTag, val.index, val.id),
        }
    }
}
crate::impl_ord!(TagFields);

#[derive(Debug, Clone, Default)]
pub struct Targets {
    pub size: u64,

    pub target_type_value: Ebml<u64>,
    pub target_type: Option<Ebml<String>>,
    pub tag_track_uid: Vec<Ebml<u64>>,
    pub tag_edition_uid: Vec<Ebml<u64>>,
    pub tag_chapter_uid: Vec<Ebml<u64>>,
    pub tag_attachment_uid: Vec<Ebml<u64>>,
}
impl Targets {
    pub fn elements(&self) -> std::collections::BTreeSet<TargetsFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(TargetsFields::TargetTypeValue(self.target_type_value.clone()));
        if let Some(el) = &self.target_type { elements.insert(TargetsFields::TargetType(el.clone())); }
        for el in &self.tag_track_uid { elements.insert(TargetsFields::TagTrackUid(el.clone())); }
        for el in &self.tag_edition_uid { elements.insert(TargetsFields::TagEditionUid(el.clone())); }
        for el in &self.tag_chapter_uid { elements.insert(TargetsFields::TagChapterUid(el.clone())); }
        for el in &self.tag_attachment_uid { elements.insert(TargetsFields::TagAttachmentUid(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum TargetsFields {
    TargetTypeValue(Ebml<u64>),
    TargetType(Ebml<String>),
    TagTrackUid(Ebml<u64>),
    TagEditionUid(Ebml<u64>),
    TagChapterUid(Ebml<u64>),
    TagAttachmentUid(Ebml<u64>),
}
impl TargetsFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::TargetTypeValue(val) => (EbmlId::TargetTypeValue, val.index, val.id),
            Self::TargetType(val) => (EbmlId::TargetType, val.index, val.id),
            Self::TagTrackUid(val) => (EbmlId::TagTrackUid, val.index, val.id),
            Self::TagEditionUid(val) => (EbmlId::TagEditionUid, val.index, val.id),
            Self::TagChapterUid(val) => (EbmlId::TagChapterUid, val.index, val.id),
            Self::TagAttachmentUid(val) => (EbmlId::TagAttachmentUid, val.index, val.id),
        }
    }
}
crate::impl_ord!(TargetsFields);

#[derive(Debug, Clone, Default)]
pub struct SimpleTag {
    pub size: u64,

    pub tag_name: Ebml<String>,
    pub tag_language: Ebml<String>,
    pub tag_language_bcp_47: Option<Ebml<String>>,
    pub tag_default: Ebml<u64>,
    pub tag_default_bogus: Ebml<u64>,
    pub tag_string: Option<Ebml<String>>,
    pub tag_binary: Option<Ebml<Vec<u8>>>,
}
impl SimpleTag {
    pub fn elements(&self) -> std::collections::BTreeSet<SimpleTagFields> {
        let mut elements = std::collections::BTreeSet::new();
        elements.insert(SimpleTagFields::TagName(self.tag_name.clone()));
        elements.insert(SimpleTagFields::TagLanguage(self.tag_language.clone()));
        if let Some(el) = &self.tag_language_bcp_47 { elements.insert(SimpleTagFields::TagLanguageBcp47(el.clone())); }
        elements.insert(SimpleTagFields::TagDefault(self.tag_default.clone()));
        elements.insert(SimpleTagFields::TagDefaultBogus(self.tag_default_bogus.clone()));
        if let Some(el) = &self.tag_string { elements.insert(SimpleTagFields::TagString(el.clone())); }
        if let Some(el) = &self.tag_binary { elements.insert(SimpleTagFields::TagBinary(el.clone())); }
        elements
    }
}
#[derive(Debug)]
pub enum SimpleTagFields {
    TagName(Ebml<String>),
    TagLanguage(Ebml<String>),
    TagLanguageBcp47(Ebml<String>),
    TagDefault(Ebml<u64>),
    TagDefaultBogus(Ebml<u64>),
    TagString(Ebml<String>),
    TagBinary(Ebml<Vec<u8>>),
}
impl SimpleTagFields {
    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {
        match self {
            Self::TagName(val) => (EbmlId::TagName, val.index, val.id),
            Self::TagLanguage(val) => (EbmlId::TagLanguage, val.index, val.id),
            Self::TagLanguageBcp47(val) => (EbmlId::TagLanguageBcp47, val.index, val.id),
            Self::TagDefault(val) => (EbmlId::TagDefault, val.index, val.id),
            Self::TagDefaultBogus(val) => (EbmlId::TagDefaultBogus, val.index, val.id),
            Self::TagString(val) => (EbmlId::TagString, val.index, val.id),
            Self::TagBinary(val) => (EbmlId::TagBinary, val.index, val.id),
        }
    }
}
crate::impl_ord!(SimpleTagFields);

