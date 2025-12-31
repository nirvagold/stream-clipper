// Video information extraction using FFmpeg

use super::types::VideoInfo;
use crate::utils::ffmpeg::get_ffmpeg_path;
use std::path::Path;
use std::process::Command;

/// Get video file information using FFmpeg
pub fn get_video_info(path: &str) -> Result<VideoInfo, String> {
    // Check file exists
    if !Path::new(path).exists() {
        return Err(format!("Video file not found: {}", path));
    }

    // Get file size
    let file_size = std::fs::metadata(path)
        .map(|m| m.len())
        .unwrap_or(0);

    // Get filename
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Run FFmpeg to get info (outputs to stderr)
    let ffmpeg = get_ffmpeg_path().map_err(|e| e.to_string())?;

    let output = Command::new(ffmpeg)
        .args(["-i", path])
        .output()
        .map_err(|e| format!("Failed to run FFmpeg: {}", e))?;

    // FFmpeg outputs info to stderr when just using -i
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Parse video info from FFmpeg output
    let (width, height, fps, codec, duration) = parse_ffmpeg_output(&stderr);

    Ok(VideoInfo {
        path: path.to_string(),
        filename,
        duration_secs: duration,
        width,
        height,
        fps,
        codec,
        file_size_bytes: file_size,
    })
}

/// Parse FFmpeg stderr output to extract video info
fn parse_ffmpeg_output(output: &str) -> (u32, u32, f32, String, f64) {
    let mut width = 0u32;
    let mut height = 0u32;
    let mut fps = 0.0f32;
    let mut codec = "unknown".to_string();
    let mut duration = 0.0f64;

    // Pre-compile regex patterns outside the loop
    let re_resolution = regex::Regex::new(r"(\d{2,5})x(\d{2,5})").ok();
    let re_frac = regex::Regex::new(r"(\d+)/(\d+)\s*fps").ok();
    let re_fps = regex::Regex::new(r"[\s,](\d+\.?\d*)\s*fps").ok();
    let re_tbr = regex::Regex::new(r"[\s,](\d+\.?\d*)\s*tbr").ok();

    // Debug: print raw output for troubleshooting
    println!("[DEBUG] FFmpeg output:\n{}", output);

    for line in output.lines() {
        // Parse duration: "Duration: 00:01:30.50, start: 0.000000"
        if line.contains("Duration:") && !line.contains("N/A") {
            if let Some(dur_part) = line.split("Duration:").nth(1) {
                if let Some(time_str) = dur_part.split(',').next() {
                    duration = parse_duration(time_str.trim());
                }
            }
        }
        
        // Parse video stream: "Stream #0:0: Video: h264 ... 1920x1080 ... 30 fps"
        // or "Stream #0:0(und): Video: h264 ... 1920x1080 [SAR 1:1 DAR 16:9], 29.97 fps, 29.97 tbr"
        if line.contains("Video:") && !line.contains("attached") {
            // Extract codec
            if let Some(codec_part) = line.split("Video:").nth(1) {
                if let Some(c) = codec_part.split_whitespace().next() {
                    codec = c.trim_end_matches(',').to_string();
                }
            }
            
            // Extract resolution (look for pattern like "1920x1080" or "1280x720")
            if let Some(ref re) = re_resolution {
                if let Some(caps) = re.captures(line) {
                    if let (Some(w), Some(h)) = (caps.get(1), caps.get(2)) {
                        width = w.as_str().parse().unwrap_or(0);
                        height = h.as_str().parse().unwrap_or(0);
                    }
                }
            }
            
            // Extract fps - multiple patterns to handle different FFmpeg outputs
            // Try fractional fps first (e.g., "30000/1001")
            if fps == 0.0 {
                if let Some(ref re) = re_frac {
                    if let Some(caps) = re.captures(line) {
                        if let (Some(num), Some(den)) = (caps.get(1), caps.get(2)) {
                            let n: f32 = num.as_str().parse().unwrap_or(0.0);
                            let d: f32 = den.as_str().parse().unwrap_or(1.0);
                            if d > 0.0 {
                                fps = n / d;
                            }
                        }
                    }
                }
            }
            
            // Try decimal fps
            if fps == 0.0 {
                if let Some(ref re) = re_fps {
                    if let Some(caps) = re.captures(line) {
                        if let Some(f) = caps.get(1) {
                            fps = f.as_str().parse().unwrap_or(0.0);
                        }
                    }
                }
            }
            
            // Fallback to tbr (timebase rate)
            if fps == 0.0 {
                if let Some(ref re) = re_tbr {
                    if let Some(caps) = re.captures(line) {
                        if let Some(f) = caps.get(1) {
                            fps = f.as_str().parse().unwrap_or(0.0);
                        }
                    }
                }
            }
            
            println!("[DEBUG] Parsed video line: {}x{}, {} fps, codec={}", width, height, fps, codec);
        }
    }

    (width, height, fps, codec, duration)
}

/// Parse duration string like "00:01:30.50" to seconds
fn parse_duration(time_str: &str) -> f64 {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 3 {
        return 0.0;
    }
    
    let hours: f64 = parts[0].parse().unwrap_or(0.0);
    let minutes: f64 = parts[1].parse().unwrap_or(0.0);
    let seconds: f64 = parts[2].parse().unwrap_or(0.0);
    
    hours * 3600.0 + minutes * 60.0 + seconds
}

/// Check if video format is supported
pub fn is_supported_format(path: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    matches!(
        ext.as_deref(),
        Some("mp4") | Some("mkv") | Some("mov") | Some("webm") | Some("avi")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_supported_format() {
        assert!(is_supported_format("video.mp4"));
        assert!(is_supported_format("video.MKV"));
        assert!(!is_supported_format("video.txt"));
    }

    #[test]
    fn test_parse_duration() {
        assert!((parse_duration("00:01:30.50") - 90.5).abs() < 0.01);
        assert!((parse_duration("01:00:00.00") - 3600.0).abs() < 0.01);
    }
}
