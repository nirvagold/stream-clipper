// Analyze commands - from spec 2-design.md Section 4.1
// Free tier: Audio detection only, max 5 clips
// Pro tier: Audio + Chat detection, unlimited clips

use crate::audio::{analyze_audio, cleanup_temp_audio, extract_audio_from_video, AudioAnalyzeSettings};
use crate::chat::{get_chat_info as get_chat_info_internal, ChatAnalyzeSettings, ChatInfo};
use crate::highlight::{merge_overlapping_highlights, score_highlights, AnalyzeResult, HighlightSettings};
use crate::license::runtime_license_check;
use crate::pro::pro_analyze_chat;
use crate::video::{get_video_info as get_video_info_internal, VideoInfo};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Emitter, Window};

// Global cancellation flag
static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

/// Combined analyze settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeSettings {
    pub audio_sensitivity: f32,
    pub audio_min_duration: f32,
    pub audio_merge_gap: f32,
    pub chat_rate_multiplier: f32,
    pub chat_window_size: f32,
    pub chat_keywords: Vec<String>,
    pub audio_weight: f32,
    pub chat_weight: f32,
    pub combo_bonus: f32,
    pub max_clips: Option<u32>,
}

impl Default for AnalyzeSettings {
    fn default() -> Self {
        Self {
            audio_sensitivity: 1.5,
            audio_min_duration: 2.0,
            audio_merge_gap: 3.0,
            chat_rate_multiplier: 3.0,
            chat_window_size: 5.0,
            chat_keywords: vec![
                "POG".to_string(), "POGGERS".to_string(), "OMG".to_string(),
                "WTF".to_string(), "CLIP IT".to_string(), "GG".to_string(),
            ],
            audio_weight: 0.6,
            chat_weight: 0.4,
            combo_bonus: 1.5,
            max_clips: None,
        }
    }
}

/// Get video file information
#[tauri::command]
pub async fn get_video_info(path: String) -> Result<VideoInfo, String> {
    get_video_info_internal(&path)
}

/// Get chat file information
#[tauri::command]
pub async fn get_chat_info(path: String) -> Result<ChatInfo, String> {
    get_chat_info_internal(&path)
}

/// Analyze video and detect highlights
#[tauri::command]
pub async fn analyze_video(
    video_path: String,
    chat_path: Option<String>,
    settings: AnalyzeSettings,
    window: Window,
) -> Result<AnalyzeResult, String> {
    // Reset cancel flag
    CANCEL_FLAG.store(false, Ordering::SeqCst);

    // Get video info for duration
    let video_info = get_video_info_internal(&video_path)?;
    println!("[DEBUG] Video info: duration={:.2}s, {}x{}", 
        video_info.duration_secs, video_info.width, video_info.height);

    // Emit progress: extracting audio
    let _ = window.emit("analyze-progress", serde_json::json!({
        "stage": "extracting",
        "progress": 0,
        "message": "Extracting audio from video..."
    }));

    // Check cancellation
    if CANCEL_FLAG.load(Ordering::SeqCst) {
        return Err("Analysis cancelled".to_string());
    }

    // Extract audio
    let wav_path = extract_audio_from_video(&video_path)
        .map_err(|e| format!("Audio extraction failed: {}", e))?;
    println!("[DEBUG] Audio extracted to: {}", wav_path);

    // Emit progress: analyzing audio
    let _ = window.emit("analyze-progress", serde_json::json!({
        "stage": "analyzing_audio",
        "progress": 30,
        "message": "Analyzing audio for highlights..."
    }));

    // Check cancellation
    if CANCEL_FLAG.load(Ordering::SeqCst) {
        cleanup_temp_audio(&wav_path);
        return Err("Analysis cancelled".to_string());
    }

    // Analyze audio
    let audio_settings = AudioAnalyzeSettings {
        sensitivity: settings.audio_sensitivity,
        min_duration: settings.audio_min_duration,
        merge_gap: settings.audio_merge_gap,
        ..Default::default()
    };

    let audio_result = analyze_audio(&wav_path, &audio_settings)?;
    println!("[DEBUG] Audio analysis: {} chunks, {} spikes, baseline={:.4}, threshold={:.4}", 
        audio_result.chunks.len(), 
        audio_result.spikes.len(),
        audio_result.baseline_rms,
        audio_result.threshold);
    
    // Get waveform data for visualization
    let waveform_data: Vec<f32> = audio_result.chunks.iter().map(|c| c.rms).collect();

    // Cleanup temp audio file
    cleanup_temp_audio(&wav_path);

    // Check if Pro user
    let is_pro = runtime_license_check();

    // Analyze chat if provided (Pro feature)
    let chat_spikes = if let Some(ref chat_file) = chat_path {
        // Emit progress: analyzing chat
        let _ = window.emit("analyze-progress", serde_json::json!({
            "stage": "analyzing_chat",
            "progress": 60,
            "message": "Analyzing chat activity..."
        }));

        // Check cancellation
        if CANCEL_FLAG.load(Ordering::SeqCst) {
            return Err("Analysis cancelled".to_string());
        }

        let chat_settings = ChatAnalyzeSettings {
            rate_multiplier: settings.chat_rate_multiplier,
            window_size: settings.chat_window_size,
            keywords: settings.chat_keywords.clone(),
            ..Default::default()
        };

        // Use Pro chat analysis (will check license internally)
        match pro_analyze_chat(chat_file, &chat_settings) {
            Ok(result) => {
                println!("[DEBUG] Chat analysis (Pro): {} spikes", result.spikes.len());
                result.spikes
            }
            Err(_) => {
                println!("[DEBUG] Chat detection skipped - Pro feature required");
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };

    // Emit progress: scoring
    let _ = window.emit("analyze-progress", serde_json::json!({
        "stage": "scoring",
        "progress": 80,
        "message": "Scoring and ranking highlights..."
    }));

    // Score highlights
    let highlight_settings = HighlightSettings {
        audio_weight: settings.audio_weight,
        chat_weight: settings.chat_weight,
        combo_bonus: settings.combo_bonus,
        max_clips: settings.max_clips,
    };

    let mut highlights = score_highlights(
        &audio_result.spikes,
        &chat_spikes,
        &highlight_settings,
    );
    println!("[DEBUG] Scored highlights: {}", highlights.len());

    // Merge overlapping highlights
    merge_overlapping_highlights(&mut highlights, 0.5);
    println!("[DEBUG] After merge: {} highlights", highlights.len());

    // Enforce max clips limit for free tier
    if !is_pro && highlights.len() > 5 {
        highlights.truncate(5);
        println!("[DEBUG] Free tier: limited to 5 clips");
    }

    // Emit progress: complete
    let _ = window.emit("analyze-progress", serde_json::json!({
        "stage": "complete",
        "progress": 100,
        "message": format!("Found {} highlights", highlights.len())
    }));

    Ok(AnalyzeResult {
        highlights,
        waveform_data,
        total_duration_secs: video_info.duration_secs,
    })
}

/// Cancel ongoing analysis
#[tauri::command]
pub async fn cancel_analysis() -> Result<(), String> {
    CANCEL_FLAG.store(true, Ordering::SeqCst);
    Ok(())
}
