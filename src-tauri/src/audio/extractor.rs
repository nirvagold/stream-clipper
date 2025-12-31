// Audio extraction from video files

use crate::utils::ffmpeg::{extract_audio, FFmpegError};
use std::path::Path;
use uuid::Uuid;

/// Extract audio from video to temporary WAV file
pub fn extract_audio_from_video(video_path: &str) -> Result<String, FFmpegError> {
    let temp_dir = std::env::temp_dir().join("stream-clipper");
    std::fs::create_dir_all(&temp_dir).map_err(|e| {
        FFmpegError::ExecutionFailed(format!("Failed to create temp dir: {}", e))
    })?;

    let temp_filename = format!("audio_{}.wav", Uuid::new_v4());
    let output_path = temp_dir.join(&temp_filename);
    let output_str = output_path.to_string_lossy().to_string();

    extract_audio(video_path, &output_str)?;

    Ok(output_str)
}

/// Clean up temporary audio file
pub fn cleanup_temp_audio(path: &str) {
    if Path::new(path).exists() {
        let _ = std::fs::remove_file(path);
    }
}

/// Get temp directory for stream clipper
pub fn get_temp_dir() -> std::path::PathBuf {
    std::env::temp_dir().join("stream-clipper")
}
