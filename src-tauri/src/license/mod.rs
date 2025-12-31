// License validation module

pub mod validator;

pub use validator::*;

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// License information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub is_pro: bool,
    pub license_key: Option<String>,
    pub activated_at: Option<String>,
    pub machine_id: String,
}

impl Default for LicenseInfo {
    fn default() -> Self {
        Self {
            is_pro: false,
            license_key: None,
            activated_at: None,
            machine_id: get_machine_id(),
        }
    }
}

/// Get unique machine identifier (hashed for privacy)
pub fn get_machine_id() -> String {
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    
    let username = whoami::username();
    
    // Add more entropy
    let platform = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    
    // Hash the combined data for privacy
    let combined = format!("{}:{}:{}:{}", username, hostname, platform, arch);
    let mut hasher = DefaultHasher::new();
    combined.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Return hex-encoded hash
    format!("{:016X}", hash)
}
