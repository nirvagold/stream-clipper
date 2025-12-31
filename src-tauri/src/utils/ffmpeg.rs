// FFmpeg wrapper utilities

use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FFmpegError {
    #[error("FFmpeg binary not found")]
    BinaryNotFound,
    #[error("FFmpeg execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Invalid output: {0}")]
    InvalidOutput(String),
}

/// Get the path to the bundled FFmpeg binary
pub fn get_ffmpeg_path() -> Result<PathBuf, FFmpegError> {
    // Try to find bundled sidecar binary
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            #[cfg(target_os = "windows")]
            let sidecar_name = "ffmpeg-x86_64-pc-windows-msvc.exe";
            #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
            let sidecar_name = "ffmpeg-x86_64-apple-darwin";
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            let sidecar_name = "ffmpeg-aarch64-apple-darwin";
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            let sidecar_name = "ffmpeg-x86_64-unknown-linux-gnu";
            #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
            let sidecar_name = "ffmpeg-aarch64-unknown-linux-gnu";
            
            let sidecar_path = exe_dir.join(sidecar_name);
            if sidecar_path.exists() {
                return Ok(sidecar_path);
            }
        }
    }

    // In development, try system FFmpeg
    if cfg!(debug_assertions) {
        if let Ok(output) = Command::new("ffmpeg").arg("-version").output() {
            if output.status.success() {
                return Ok(PathBuf::from("ffmpeg"));
            }
        }
    }

    // Fallback to system FFmpeg
    Ok(PathBuf::from("ffmpeg"))
}

/// Extract audio from video file to WAV format
pub fn extract_audio(input_path: &str, output_path: &str) -> Result<(), FFmpegError> {
    let ffmpeg = get_ffmpeg_path()?;
    
    let output = Command::new(ffmpeg)
        .args([
            "-i", input_path,
            "-vn",                    // No video
            "-acodec", "pcm_s16le",   // PCM 16-bit
            "-ar", "22050",           // 22kHz sample rate
            "-ac", "1",               // Mono
            "-y",                     // Overwrite output
            output_path,
        ])
        .output()
        .map_err(|e| FFmpegError::ExecutionFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(FFmpegError::ExecutionFailed(stderr.to_string()));
    }

    Ok(())
}

/// Cut a clip from video
pub fn cut_clip(
    input_path: &str,
    output_path: &str,
    start_secs: f64,
    duration_secs: f64,
    reencode: bool,
) -> Result<(), FFmpegError> {
    let ffmpeg = get_ffmpeg_path()?;
    
    let start_str = format!("{:.3}", start_secs);
    let duration_str = format!("{:.3}", duration_secs);
    
    let mut args = vec![
        "-ss".to_string(), start_str,
        "-i".to_string(), input_path.to_string(),
        "-t".to_string(), duration_str,
    ];
    
    if reencode {
        args.extend([
            "-c:v".to_string(), "libx264".to_string(),
            "-preset".to_string(), "fast".to_string(),
            "-crf".to_string(), "23".to_string(),
            "-c:a".to_string(), "aac".to_string(),
            "-b:a".to_string(), "128k".to_string(),
        ]);
    } else {
        args.extend(["-c".to_string(), "copy".to_string()]);
    }
    
    args.extend(["-y".to_string(), output_path.to_string()]);
    
    let output = Command::new(ffmpeg)
        .args(&args)
        .output()
        .map_err(|e| FFmpegError::ExecutionFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(FFmpegError::ExecutionFailed(stderr.to_string()));
    }

    Ok(())
}

/// Get video duration using ffmpeg
pub fn get_video_duration(input_path: &str) -> Result<f64, FFmpegError> {
    let ffmpeg = get_ffmpeg_path()?;
    
    let output = Command::new(&ffmpeg)
        .args([
            "-i", input_path,
            "-f", "null",
            "-"
        ])
        .output()
        .map_err(|e| FFmpegError::ExecutionFailed(e.to_string()))?;

    // FFmpeg outputs duration info to stderr
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Parse duration from output like "Duration: 00:01:30.50"
    for line in stderr.lines() {
        if line.contains("Duration:") {
            if let Some(duration_str) = line.split("Duration:").nth(1) {
                if let Some(time_str) = duration_str.split(',').next() {
                    let time_str = time_str.trim();
                    return parse_duration(time_str);
                }
            }
        }
    }
    
    Err(FFmpegError::InvalidOutput("Could not parse duration".to_string()))
}

/// Parse duration string like "00:01:30.50" to seconds
fn parse_duration(time_str: &str) -> Result<f64, FFmpegError> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 3 {
        return Err(FFmpegError::InvalidOutput(format!("Invalid time format: {}", time_str)));
    }
    
    let hours: f64 = parts[0].parse().unwrap_or(0.0);
    let minutes: f64 = parts[1].parse().unwrap_or(0.0);
    let seconds: f64 = parts[2].parse().unwrap_or(0.0);
    
    Ok(hours * 3600.0 + minutes * 60.0 + seconds)
}
