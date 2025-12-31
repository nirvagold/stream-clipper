// Pro: Video effects - obfuscated (vertical crop, fade, high-res, no watermark)
#![allow(non_snake_case)]

use crate::license::runtime_license_check;

/// Pro video filter builder - returns FFmpeg filter string
#[inline(never)]
pub fn pro_build_filters(
    h: Option<u32>,    // height
    vc: bool,          // vertical crop
    fe: bool,          // fade effect
    dur: f64,          // duration
) -> Result<Vec<String>, String> {
    if !runtime_license_check() { return Err(_e()); }
    let mut f = Vec::new();
    
    // High resolution (Pro: up to 4K)
    if let Some(ht) = h { f.push(format!("scale=-2:{}", ht)); }
    
    // Vertical crop 9:16
    if vc { f.push(_vc()); }
    
    // Fade effects
    if fe { f.extend(_fe(dur)); }
    
    Ok(f)
}

/// Check if resolution is Pro-only
#[inline(never)]
pub fn pro_resolution_allowed(h: u32) -> bool {
    if h <= 720 { return true; } // Free tier
    runtime_license_check() // Pro required for >720p
}

/// Check if format is Pro-only
#[inline(never)]
pub fn pro_format_allowed(fmt: &str) -> bool {
    if fmt == "mp4" { return true; } // Free tier
    runtime_license_check() // Pro required for webm
}

/// Check if watermark should be skipped (Pro only)
#[inline(never)]
pub fn pro_skip_watermark() -> bool {
    runtime_license_check()
}

// Internal obfuscated helpers
#[inline(never)] fn _vc() -> String { 
    // "crop=ih*9/16:ih"
    String::from_utf8_lossy(&[99,114,111,112,61,105,104,42,57,47,49,54,58,105,104]).to_string()
}

#[inline(never)] fn _fe(d: f64) -> Vec<String> {
    vec![
        format!("fade=t=in:st=0:d=0.5"),
        format!("fade=t=out:st={:.3}:d=0.5", d - 0.5)
    ]
}

#[inline(never)] fn _e() -> String {
    String::from_utf8_lossy(&[80,114,111,32,108,105,99,101,110,115,101,32,114,101,113,117,105,114,101,100]).to_string()
}
