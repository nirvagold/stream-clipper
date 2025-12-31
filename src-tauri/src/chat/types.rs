// Chat analysis types - from spec 2-design.md Section 2.1

use serde::{Deserialize, Serialize};

/// Chat file format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatFormat {
    TwitchJson,
    YouTubeJson,
    GenericTxt,
    Unknown,
}

/// Single chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub timestamp_secs: f64,
    pub username: String,
    pub message: String,
    pub has_emote: bool,
}

/// Chat analysis window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatWindow {
    pub start_secs: f64,
    pub end_secs: f64,
    pub message_count: u32,
    pub unique_users: u32,
    pub keyword_matches: u32,
    pub emote_density: f32,
    pub caps_ratio: f32,
}

/// Detected chat activity spike
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSpike {
    pub start_secs: f64,
    pub end_secs: f64,
    pub peak_rate: f32,
    pub keyword_score: f32,
    pub score: f32, // Normalized 0-100
}

/// Complete chat analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatAnalysisResult {
    pub windows: Vec<ChatWindow>,
    pub baseline_rate: f32,
    pub threshold: f32,
    pub spikes: Vec<ChatSpike>,
}

/// Chat file info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatInfo {
    pub path: String,
    pub format: ChatFormat,
    pub total_messages: u32,
    pub duration_secs: f64,
    pub avg_rate_per_min: f32,
}

/// Settings for chat analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatAnalyzeSettings {
    pub rate_multiplier: f32,    // 1.0 - 5.0, default 3.0
    pub window_size: f32,        // seconds, default 5.0
    pub keywords: Vec<String>,
    pub keyword_threshold: u32,  // minimum keyword matches
    pub emote_threshold: f32,    // minimum emote density
}

impl Default for ChatAnalyzeSettings {
    fn default() -> Self {
        Self {
            rate_multiplier: 3.0,
            window_size: 5.0,
            keywords: vec![
                "POG".to_string(),
                "POGGERS".to_string(),
                "POGU".to_string(),
                "LETS GO".to_string(),
                "LET'S GO".to_string(),
                "OMG".to_string(),
                "WTF".to_string(),
                "CLIP IT".to_string(),
                "CLIP THAT".to_string(),
                "GG".to_string(),
                "GGWP".to_string(),
                "HOLY".to_string(),
                "INSANE".to_string(),
                "CRAZY".to_string(),
                "NO WAY".to_string(),
                "NOWAY".to_string(),
                "KEKW".to_string(),
                "LULW".to_string(),
                "OMEGALUL".to_string(),
                "HYPE".to_string(),
            ],
            keyword_threshold: 3,
            emote_threshold: 0.3,
        }
    }
}
