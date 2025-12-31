// License commands - from spec 2-design.md Section 4.4

use crate::license::{
    activate_license as activate_license_internal,
    deactivate_license as deactivate_license_internal,
    load_license,
    LicenseInfo,
};

/// Get current license status
#[tauri::command]
pub fn get_license_status() -> LicenseInfo {
    load_license()
}

/// Validate and activate license key
#[tauri::command]
pub fn activate_license(key: String) -> Result<LicenseInfo, String> {
    activate_license_internal(&key)
}

/// Deactivate current license
#[tauri::command]
pub fn deactivate_license() -> Result<(), String> {
    deactivate_license_internal()
}
