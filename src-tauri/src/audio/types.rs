// Audio analysis types - from spec 2-design.md Section 2.1

use serde::{Deserialize, Serialize};

/// Single audio chunk with volume data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioChunk {
    pub start_secs: f64,
    pub end_secs: f64,
    pub rms: f32,   // Root Mean Square volume
    pub peak: f32,  // Peak amplitude
}

/// Detected audio spike (loud moment)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSpike {
    pub start_secs: f64,
    pub end_secs: f64,
    pub peak_rms: f32,
    pub score: f32, // Normalized 0-100
}

/// Complete audio analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAnalysisResult {
    pub chunks: Vec<AudioChunk>,
    pub baseline_rms: f32,
    pub std_dev: f32,
    pub threshold: f32,
    pub spikes: Vec<AudioSpike>,
}

/// Settings for audio analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAnalyzeSettings {
    pub sensitivity: f32,      // 1.0 - 4.0, default 2.0
    pub min_duration: f32,     // seconds, default 2.0
    pub merge_gap: f32,        // seconds, default 3.0
    pub chunk_duration: f32,   // seconds, default 0.5
    pub sample_rate: u32,      // Hz, default 22050
}

impl Default for AudioAnalyzeSettings {
    fn default() -> Self {
        Self {
            sensitivity: 2.0,
            min_duration: 2.0,
            merge_gap: 3.0,
            chunk_duration: 0.5,
            sample_rate: 22050,
        }
    }
}
