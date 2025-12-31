// Video clipping using FFmpeg
// Free tier: 720p max, watermark, MP4 only
// Pro tier: Up to 4K, no watermark, vertical crop, fade effects, WebM

use super::types::{ClipExport, ExportResult, ExportSettings, OutputFormat};
use crate::pro::{pro_build_filters, pro_skip_watermark, pro_format_allowed, pro_resolution_allowed};
use crate::utils::ffmpeg::{get_ffmpeg_path, FFmpegError};
use crate::utils::time::format_timestamp;
use std::path::Path;
use std::process::Command;

/// Export a single clip
/// Free: 720p, watermark, MP4
/// Pro: Up to 4K, no watermark, vertical crop, fade, WebM
pub fn export_clip(
    video_path: &str,
    clip: &ClipExport,
    settings: &ExportSettings,
    video_filename: &str,
) -> ExportResult {
    // Apply padding (add context before and after the highlight)
    let padded_start = (clip.start_secs - settings.padding_before as f64).max(0.0);
    let padded_end = clip.end_secs + settings.padding_after as f64;
    let duration = padded_end - padded_start;

    // Generate output filename
    let output_filename = generate_filename(video_filename, clip, settings);
    let output_path = Path::new(&settings.output_folder).join(&output_filename);
    let output_str = output_path.to_string_lossy().to_string();

    // Build FFmpeg command with tier-appropriate settings
    match build_and_run_ffmpeg(video_path, &output_str, padded_start, duration, settings) {
        Ok(_) => ExportResult {
            highlight_id: clip.highlight_id,
            output_path: output_str,
            success: true,
            error: None,
        },
        Err(e) => ExportResult {
            highlight_id: clip.highlight_id,
            output_path: output_str,
            success: false,
            error: Some(e.to_string()),
        },
    }
}

/// Generate output filename
fn generate_filename(video_filename: &str, clip: &ClipExport, settings: &ExportSettings) -> String {
    let base_name = Path::new(video_filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("clip");

    let timestamp = format_timestamp(clip.start_secs);
    
    // Format: Free=MP4 only, Pro=MP4 or WebM
    let ext = match settings.format {
        OutputFormat::Mp4 => "mp4",
        OutputFormat::WebM => {
            if pro_format_allowed("webm") { "webm" } else { "mp4" }
        }
    };

    format!("{}_clip{:02}_{}.{}", base_name, clip.highlight_id, timestamp, ext)
}

/// Build and run FFmpeg command
fn build_and_run_ffmpeg(
    input_path: &str,
    output_path: &str,
    start_secs: f64,
    duration: f64,
    settings: &ExportSettings,
) -> Result<(), FFmpegError> {
    let ffmpeg = get_ffmpeg_path()?;

    let mut args: Vec<String> = vec![
        "-ss".to_string(),
        format!("{:.3}", start_secs),
        "-i".to_string(),
        input_path.to_string(),
        "-t".to_string(),
        format!("{:.3}", duration),
    ];

    // Build video filter chain
    let mut vfilters: Vec<String> = Vec::new();

    // Resolution: Free=720p max, Pro=up to 4K
    let height = settings.resolution.to_height();
    let effective_height = match height {
        Some(h) if pro_resolution_allowed(h) => Some(h),
        Some(_) => Some(720), // Fallback to 720p for free tier
        None => {
            // Source resolution - limit to 720p for free
            if pro_resolution_allowed(1080) { None } else { Some(720) }
        }
    };
    
    if let Some(h) = effective_height {
        vfilters.push(format!("scale=-2:{}", h));
    }

    // Pro features: vertical crop, fade effects
    let skip_watermark = pro_skip_watermark();
    
    if settings.vertical_crop || settings.fade_effect {
        if let Ok(pro_filters) = pro_build_filters(
            None, // Already handled resolution above
            settings.vertical_crop,
            settings.fade_effect,
            duration,
        ) {
            vfilters.extend(pro_filters);
        }
        // If Pro check fails, these filters are simply not added (graceful degradation)
    }

    // Watermark: Free tier only
    if !skip_watermark {
        let watermark = "drawtext=text='Stream Clipper':fontsize=24:fontcolor=white@0.7:x=w-tw-20:y=h-th-20:shadowcolor=black@0.5:shadowx=2:shadowy=2";
        vfilters.push(watermark.to_string());
    }

    // Apply video filters
    if !vfilters.is_empty() {
        args.push("-vf".to_string());
        args.push(vfilters.join(","));
    }

    // Format: Free=MP4 only, Pro=MP4 or WebM
    let use_webm = matches!(settings.format, OutputFormat::WebM) && pro_format_allowed("webm");
    
    if use_webm {
        args.extend([
            "-c:v".to_string(), "libvpx-vp9".to_string(),
            "-crf".to_string(), "30".to_string(),
            "-b:v".to_string(), "0".to_string(),
            "-c:a".to_string(), "libopus".to_string(),
            "-b:a".to_string(), "128k".to_string(),
        ]);
    } else {
        args.extend([
            "-c:v".to_string(), "libx264".to_string(),
            "-preset".to_string(), "fast".to_string(),
            "-crf".to_string(), "23".to_string(),
            "-c:a".to_string(), "aac".to_string(),
            "-b:a".to_string(), "128k".to_string(),
        ]);
    }

    // Overwrite output
    args.push("-y".to_string());
    args.push(output_path.to_string());

    println!("[DEBUG] FFmpeg args: {:?}", args);

    // Run FFmpeg
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

/// Ensure output folder exists
pub fn ensure_output_folder(path: &str) -> Result<(), String> {
    std::fs::create_dir_all(path).map_err(|e| format!("Failed to create output folder: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_filename() {
        let clip = ClipExport {
            highlight_id: 1,
            start_secs: 3665.0, // 1:01:05
            end_secs: 3680.0,
        };
        let settings = ExportSettings::default();

        let filename = generate_filename("stream_2024.mp4", &clip, &settings);
        assert!(filename.contains("stream_2024"));
        assert!(filename.contains("clip01"));
        assert!(filename.ends_with(".mp4"));
    }
}
