// Stream Clipper - Main Library
// Auto-detect highlight moments from streaming videos

pub mod audio;
pub mod chat;
pub mod commands;
pub mod highlight;
pub mod license;
pub mod pro;  // Pro features (obfuscated)
pub mod utils;
pub mod video;

use commands::{analyze, config, export, license as license_cmd};

/// Cleanup temporary files on app exit
fn cleanup_on_exit() {
    let temp_dir = std::env::temp_dir().join("stream-clipper");
    if temp_dir.exists() {
        // Remove entire temp directory
        if let Err(e) = std::fs::remove_dir_all(&temp_dir) {
            eprintln!("[CLEANUP] Failed to remove temp dir: {}", e);
        } else {
            println!("[CLEANUP] Temp files cleaned up");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            // Analyze commands
            analyze::get_video_info,
            analyze::get_chat_info,
            analyze::analyze_video,
            analyze::cancel_analysis,
            // Export commands
            export::export_clips,
            export::preview_clip,
            export::cancel_export,
            export::open_folder,
            export::cleanup_temp_files,
            // Config commands
            config::get_settings,
            config::save_settings,
            config::reset_settings,
            config::get_default_output_folder,
            config::pick_folder,
            config::pick_file,
            // License commands
            license_cmd::get_license_status,
            license_cmd::activate_license,
            license_cmd::deactivate_license,
        ])
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                cleanup_on_exit();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
