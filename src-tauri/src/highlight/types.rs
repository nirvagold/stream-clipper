// Highlight types - from spec 2-design.md Section 2.1

use serde::{Deserialize, Serialize};

/// Type of highlight source
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HighlightType {
    Audio,
    Chat,
    Combo, // Both audio and chat
}

/// A detected highlight moment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    pub id: u32,
    pub start_secs: f64,
    pub end_secs: f64,
    pub duration_secs: f64,
    pub highlight_type: HighlightType,
    pub score: f32, // 0-100
    pub audio_score: Option<f32>,
    pub chat_score: Option<f32>,
    pub reasons: Vec<String>,
}

/// Settings for highlight scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightSettings {
    pub audio_weight: f32,    // 0.0 - 1.0, default 0.6
    pub chat_weight: f32,     // 0.0 - 1.0, default 0.4
    pub combo_bonus: f32,     // multiplier, default 1.5
    pub max_clips: Option<u32>, // None = unlimited
}

impl Default for HighlightSettings {
    fn default() -> Self {
        Self {
            audio_weight: 0.6,
            chat_weight: 0.4,
            combo_bonus: 1.5,
            max_clips: None,
        }
    }
}

/// Complete analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResult {
    pub highlights: Vec<Highlight>,
    pub waveform_data: Vec<f32>, // For visualization
    pub total_duration_secs: f64,
}
