// Video types - from spec 2-design.md Section 2.1

use serde::{Deserialize, Serialize};

/// Video file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub path: String,
    pub filename: String,
    pub duration_secs: f64,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub codec: String,
    pub file_size_bytes: u64,
}

/// Clip to export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipExport {
    pub highlight_id: u32,
    pub start_secs: f64, // With padding applied
    pub end_secs: f64,   // With padding applied
}

/// Export result for a single clip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub highlight_id: u32,
    pub output_path: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Output format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    Mp4,
    WebM,
}

/// Output resolution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputResolution {
    R720p,
    R1080p,
    R1440p,
    R4K,
    Source,
}

impl OutputResolution {
    pub fn to_height(&self) -> Option<u32> {
        match self {
            OutputResolution::R720p => Some(720),
            OutputResolution::R1080p => Some(1080),
            OutputResolution::R1440p => Some(1440),
            OutputResolution::R4K => Some(2160),
            OutputResolution::Source => None,
        }
    }
}

/// Export settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettings {
    pub output_folder: String,
    pub format: OutputFormat,
    pub resolution: OutputResolution,
    pub padding_before: f32, // seconds
    pub padding_after: f32,  // seconds
    pub vertical_crop: bool,
    pub fade_effect: bool,
    pub add_watermark: bool,
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            output_folder: String::new(),
            format: OutputFormat::Mp4,
            resolution: OutputResolution::R1080p,
            padding_before: 3.0,
            padding_after: 2.0,
            vertical_crop: false,
            fade_effect: false,
            add_watermark: false,
        }
    }
}
