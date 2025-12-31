// Export commands - from spec 2-design.md Section 4.2

use crate::video::{export_clip, ensure_output_folder, ClipExport, ExportResult, ExportSettings};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Emitter, Window};

// Global cancellation flag
static CANCEL_EXPORT_FLAG: AtomicBool = AtomicBool::new(false);

/// Export selected clips
#[tauri::command]
pub async fn export_clips(
    video_path: String,
    clips: Vec<ClipExport>,
    settings: ExportSettings,
    window: Window,
) -> Result<Vec<ExportResult>, String> {
    // Reset cancel flag
    CANCEL_EXPORT_FLAG.store(false, Ordering::SeqCst);

    // Ensure output folder exists
    ensure_output_folder(&settings.output_folder)?;

    // Get video filename for output naming
    let video_filename = std::path::Path::new(&video_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("video")
        .to_string();

    let total = clips.len();
    let mut results = Vec::new();

    for (i, clip) in clips.iter().enumerate() {
        // Check cancellation
        if CANCEL_EXPORT_FLAG.load(Ordering::SeqCst) {
            return Err("Export cancelled".to_string());
        }

        // Emit progress
        let progress = ((i as f32 / total as f32) * 100.0) as u32;
        let _ = window.emit("export-progress", serde_json::json!({
            "current": i + 1,
            "total": total,
            "percent": progress,
            "message": format!("Exporting clip {}/{}...", i + 1, total)
        }));

        // Export clip
        let result = export_clip(&video_path, clip, &settings, &video_filename);
        results.push(result);
    }

    // Emit completion
    let _ = window.emit("export-progress", serde_json::json!({
        "current": total,
        "total": total,
        "percent": 100,
        "message": format!("Exported {} clips", total)
    }));

    Ok(results)
}

/// Generate preview for a single clip
#[tauri::command]
pub async fn preview_clip(
    video_path: String,
    start_secs: f64,
    end_secs: f64,
) -> Result<String, String> {
    // Create temp preview directory
    let temp_dir = std::env::temp_dir().join("stream-clipper").join("previews");
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create preview directory: {}", e))?;

    // Create minimal export settings for preview
    // Apply default padding (3s before, 2s after) for context
    let settings = ExportSettings {
        output_folder: temp_dir.to_string_lossy().to_string(),
        padding_before: 3.0,
        padding_after: 2.0,
        ..Default::default()
    };

    let clip = ClipExport {
        highlight_id: 0,
        start_secs,
        end_secs,
    };

    let result = export_clip(&video_path, &clip, &settings, "preview");

    if result.success {
        Ok(result.output_path)
    } else {
        Err(result.error.unwrap_or_else(|| "Preview generation failed".to_string()))
    }
}

/// Cancel ongoing export
#[tauri::command]
pub async fn cancel_export() -> Result<(), String> {
    CANCEL_EXPORT_FLAG.store(true, Ordering::SeqCst);
    Ok(())
}

/// Cleanup temporary files (previews and extracted audio)
#[tauri::command]
pub fn cleanup_temp_files() -> Result<u64, String> {
    let temp_dir = std::env::temp_dir().join("stream-clipper");
    
    if !temp_dir.exists() {
        return Ok(0);
    }

    let mut cleaned_bytes: u64 = 0;

    // Clean previews folder
    let previews_dir = temp_dir.join("previews");
    if previews_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&previews_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    cleaned_bytes += metadata.len();
                }
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }

    // Clean audio files (*.wav)
    if let Ok(entries) = std::fs::read_dir(&temp_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "wav") {
                if let Ok(metadata) = entry.metadata() {
                    cleaned_bytes += metadata.len();
                }
                let _ = std::fs::remove_file(path);
            }
        }
    }

    Ok(cleaned_bytes)
}

/// Open folder in system file explorer
#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    Ok(())
}
