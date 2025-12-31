// Config commands - from spec 2-design.md Section 4.3

use crate::commands::analyze::AnalyzeSettings;
use crate::video::ExportSettings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Combined application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub version: u32,
    pub detection: AnalyzeSettings,
    pub export: ExportSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: 1,
            detection: AnalyzeSettings::default(),
            export: ExportSettings {
                output_folder: get_default_output_folder(),
                ..Default::default()
            },
        }
    }
}

/// Get config file path
fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("stream-clipper");

    config_dir.join("config.json")
}

/// Get current settings
#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    let path = get_config_path();

    if path.exists() {
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))
    } else {
        Ok(Settings::default())
    }
}

/// Save settings
#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<(), String> {
    let path = get_config_path();

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&path, content)
        .map_err(|e| format!("Failed to write config: {}", e))?;

    Ok(())
}

/// Reset settings to default
#[tauri::command]
pub fn reset_settings() -> Result<Settings, String> {
    let settings = Settings::default();
    save_settings(settings.clone())?;
    Ok(settings)
}

/// Get default output folder
#[tauri::command]
pub fn get_default_output_folder() -> String {
    dirs::video_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("StreamClipper")
        .to_string_lossy()
        .to_string()
}

/// File filter for dialog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

/// Open folder picker dialog
#[tauri::command]
pub async fn pick_folder(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    
    let (tx, rx) = std::sync::mpsc::channel();
    
    app.dialog()
        .file()
        .set_title("Select Output Folder")
        .pick_folder(move |path| {
            let _ = tx.send(path);
        });

    match rx.recv() {
        Ok(Some(path)) => Ok(Some(path.to_string())),
        Ok(None) => Ok(None),
        Err(_) => Ok(None),
    }
}

/// Open file picker dialog
#[tauri::command]
pub async fn pick_file(app: tauri::AppHandle, filters: Vec<FileFilter>) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    
    let (tx, rx) = std::sync::mpsc::channel();
    
    let mut dialog = app.dialog().file().set_title("Select File");

    for filter in filters {
        let extensions: Vec<&str> = filter.extensions.iter().map(|s| s.as_str()).collect();
        dialog = dialog.add_filter(&filter.name, &extensions);
    }

    dialog.pick_file(move |path| {
        let _ = tx.send(path);
    });

    match rx.recv() {
        Ok(Some(path)) => Ok(Some(path.to_string())),
        Ok(None) => Ok(None),
        Err(_) => Ok(None),
    }
}
