#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FFProbeRaw {
    pub program_version: Option<ProgramVersion>,
    pub library_versions: Option<LibraryVersions>,
    pub pixel_formats: Option<PixelFormats>,
    pub packets: Option<Packets>,
    pub frames: Option<Frames>,
    pub packets_and_frames: Option<PacketsAndFrames>,
    pub programs: Option<Programs>,
    pub streams: Option<Streams>,
    pub chapters: Option<Chapters>,
    pub format: Option<Format>,
    pub error: Option<FFProbeError>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProgramVersion {
    pub version: String,
    pub copyright: String,
    pub build_date: Option<String>,
    pub build_time: Option<String>,
    pub compiler_ident: String,
    pub configuration: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LibraryVersions {
    #[serde(rename = "library_version", default)]
    pub library_versions: Vec<LibraryVersion>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LibraryVersion {
    pub name: String,
    pub major: i32,
    pub minor: i32,
    pub micro: i32,
    pub version: i32,
    pub ident: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PixelFormats {
    #[serde(rename = "pixel_format", default)]
    pub pixel_formats: Vec<PixelFormatRaw>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PixelFormatRaw {
    pub name: String,
    pub nb_components: i32,
    pub log2_chroma_w: Option<i32>,
    pub log2_chroma_h: Option<i32>,
    pub bits_per_pixel: Option<i32>,

    pub flags: Option<PixelFormatFlags>,
    pub components: Option<PixelFormatComponents>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PixelFormatFlags {
    pub big_endian: i32,
    pub palette: i32,
    pub bitstream: i32,
    pub hwaccel: i32,
    pub planar: i32,
    pub rgb: i32,
    pub alpha: i32,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PixelFormatComponents {
    #[serde(rename = "component", default)]
    pub components: Vec<PixelFormatComponent>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PixelFormatComponent {
    pub index: i32,
    pub bit_depth: i32,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Packets {
    #[serde(rename = "packet", default)]
    pub packets: Vec<PacketRaw>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PacketRaw {
    pub codec_type: String,
    pub stream_index: i32,
    pub pts: Option<i64>,
    pub pts_time: Option<f32>,
    pub dts: Option<i64>,
    pub dts_time: Option<f32>,
    pub duration: Option<i64>,
    pub duration_time: Option<f32>,
    pub size: i64,
    pub pos: Option<i64>,
    pub flags: String,
    pub data: Option<String>,
    pub data_hash: Option<String>,

    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
    pub side_data_list: Option<PacketSideDataList>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    key: String,
    value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PacketSideDataList {
    #[serde(rename = "side_data", default)]
    pub side_data: Vec<PacketSideData>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PacketSideData {
    pub side_data_type: Option<String>,
    pub side_data_size: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Frames {
    #[serde(rename = "$value", default)]
    pub frames: Vec<FramesEnum>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FramesEnum {
    #[serde(rename = "frame")]
    Frame(FrameRaw),
    #[serde(rename = "subtitle")]
    Subtitle(Subtitle),
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrameRaw {
    pub media_type: String,
    pub stream_index: Option<i32>,
    pub key_frame: i32,
    pub pts: Option<i64>,
    pub pts_time: Option<f32>,
    pub pkt_pts: Option<i64>,
    pub pkt_pts_time: Option<f32>,
    pub pkt_dts: Option<i64>,
    pub pkt_dts_time: Option<f32>,
    pub best_effort_timestamp: Option<i64>,
    pub best_effort_timestamp_time: Option<f32>,
    pub pkt_duration: Option<i64>,
    pub pkt_duration_time: Option<f32>,
    pub pkt_pos: Option<i64>,
    pub pkt_size: Option<i32>,

    // Audio attributes
    pub sample_fmt: Option<String>,
    pub nb_samples: Option<i64>,
    pub channels: Option<i32>,
    pub channel_layout: Option<String>,

    // Video attributes
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub pix_fmt: Option<String>,
    pub sample_aspect_ratio: Option<String>,
    pub pict_type: Option<String>,
    pub coded_picture_number: Option<i64>,
    pub display_picture_number: Option<i64>,
    pub interlaced_frame: Option<i32>,
    pub top_field_first: Option<i32>,
    pub repeat_pict: Option<i32>,
    pub color_range: Option<String>,
    pub color_space: Option<String>,
    pub color_primaries: Option<String>,
    pub color_transfer: Option<String>,
    pub chroma_location: Option<String>,

    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
    pub logs: Option<Logs>,
    pub side_data_list: Option<FrameSideDataList>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Logs {
    #[serde(rename = "log", default)]
    pub logs: Vec<Log>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Log {
    pub context: Option<String>,
    pub level: Option<i32>,
    pub category: Option<i32>,
    pub parent_context: Option<String>,
    pub parent_category: Option<i32>,
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrameSideDataList {
    #[serde(rename = "side_data", default)]
    pub side_data: Vec<FrameSideDataRaw>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrameSideDataRaw {
    pub side_data_type: Option<String>,
    pub side_data_size: Option<i32>,
    pub timecode: Option<String>,

    pub timecodes: Option<FrameSideDataTimecodeList>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrameSideDataTimecodeList {
    #[serde(rename = "timecode", default)]
    pub timecodes: Vec<FrameSideDataTimecode>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrameSideDataTimecode {
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Subtitle {
    pub media_type: String,
    pub pts: Option<i64>,
    pub pts_time: Option<f32>,
    pub format: Option<i32>,
    pub start_display_time: Option<i32>,
    pub end_display_time: Option<i32>,
    pub num_rects: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PacketsAndFrames {
    #[serde(rename = "$value", default)]
    pub packets_and_frames: Vec<PacketsAndFramesEnum>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PacketsAndFramesEnum {
    #[serde(rename = "packet")]
    Packet(PacketRaw),
    #[serde(rename = "frame")]
    Frame(Box<FrameRaw>),
    #[serde(rename = "subtitle")]
    Subtitle(Subtitle),
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Programs {
    #[serde(rename = "program", default)]
    programs: Vec<ProgramRaw>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProgramRaw {
    pub program_id: i32,
    pub program_num: i32,
    pub nb_streams: i32,
    pub start_time: Option<f32>,
    pub start_pts: Option<i64>,
    pub end_time: Option<f32>,
    pub end_pts: Option<i64>,
    pub pmt_pid: i32,
    pub pcr_pid: i32,

    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
    pub streams: Option<Streams>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Streams {
    #[serde(rename = "stream", default)]
    pub streams: Vec<StreamRaw>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StreamRaw {
    pub index: i32,
    pub codec_name: Option<String>,
    pub codec_long_name: Option<String>,
    pub profile: Option<String>,
    pub codec_type: Option<String>,
    pub codec_tag: String,
    pub codec_tag_string: String,
    pub extradata: Option<String>,
    pub extradata_hash: Option<String>,

    // Video attributes
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub coded_width: Option<i32>,
    pub coded_height: Option<i32>,
    pub closed_captions: Option<bool>,
    pub has_b_frames: Option<i32>,
    pub sample_aspect_ratio: Option<String>,
    pub display_aspect_ratio: Option<String>,
    pub pix_fmt: Option<String>,
    pub level: Option<i32>,
    pub color_range: Option<String>,
    pub color_space: Option<String>,
    pub color_transfer: Option<String>,
    pub color_primaries: Option<String>,
    pub chroma_location: Option<String>,
    pub field_order: Option<String>,
    pub refs: Option<i32>,

    // Audio attributes
    pub sample_fmt: Option<String>,
    pub sample_rate: Option<i32>,
    pub channels: Option<i32>,
    pub channel_layout: Option<String>,
    pub bits_per_sample: Option<i32>,

    pub id: Option<String>,
    pub r_frame_rate: String,
    pub avg_frame_rate: String,
    pub time_base: String,
    pub start_pts: Option<i64>,
    pub start_time: Option<f32>,
    pub duration_ts: Option<i64>,
    pub duration: Option<f32>,
    pub bit_rate: Option<i32>,
    pub max_bit_rate: Option<i32>,
    pub bits_per_raw_sample: Option<i32>,
    pub nb_frames: Option<i32>,
    pub nb_read_frames: Option<i32>,
    pub nb_read_packets: Option<i32>,


    pub disposition: Option<StreamDisposition>,
    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
    pub side_data_list: Option<PacketSideDataList>
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StreamDisposition {
    pub default: i32,
    pub dub: i32,
    pub original: i32,
    pub comment: i32,
    pub lyrics: i32,
    pub karaoke: i32,
    pub forced: i32,
    pub hearing_impaired: i32,
    pub visual_impaired: i32,
    pub clean_effects: i32,
    pub attached_pic: i32,
    pub timed_thumbnails: i32,
    pub captions: Option<i32>,
    pub descriptions: Option<i32>,
    pub metadata: Option<i32>,
    pub dependent: Option<i32>,
    pub still_image: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Chapters {
    #[serde(rename = "chapter", default)]
    pub chapters: Vec<Chapter>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub id: i32,
    pub time_base: String,
    pub start: i32,
    pub start_time: Option<f32>,
    pub end: i32,
    pub end_time: f32,

    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Format {
    pub filename: String,
    pub nb_streams: i32,
    pub nb_programs: i32,
    pub format_name: String,
    pub format_long_name: Option<String>,
    pub start_time: Option<f32>,
    pub duration: Option<f32>,
    pub size: Option<i64>,
    pub bit_rate: Option<i64>,
    pub probe_score: Option<i32>,

    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FFProbeError {
    pub code: i32,
    pub string: String,
}
