// License key validation with anti-crack measures

use super::LicenseInfo;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

// ============================================
// OBFUSCATED CONSTANTS (Anti-pattern matching)
// ============================================

const fn _x(v: u32) -> u32 { v ^ 0xDEADBEEF }
const fn _y(v: u64) -> u64 { v ^ 0xCAFEBABE12345678 }

const _M1: u32 = _x(0x53545243);
#[allow(dead_code)]
const _M2: u32 = _x(0x4C495052);
#[allow(dead_code)]
const _M3: u32 = _x(0x0000FFFF ^ 0xDEADBEEF);

// Obfuscated string builder - harder to find in binary
macro_rules! _s {
    ($($b:expr),*) => {{
        let bytes: &[u8] = &[$($b),*];
        String::from_utf8_lossy(bytes).to_string()
    }};
}

// ============================================
// LICENSE FILE HANDLING
// ============================================

#[inline(never)]
fn _p() -> PathBuf {
    let d = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(_s!(115, 116, 114, 101, 97, 109, 45, 99, 108, 105, 112, 112, 101, 114)); // "stream-clipper"
    d.join(_s!(46, 108, 105, 99, 101, 110, 115, 101, 46, 100, 97, 116)) // ".license.dat"
}

#[inline(never)]
pub fn load_license() -> LicenseInfo {
    let p = _p();
    if p.exists() {
        if let Ok(e) = fs::read(&p) {
            if let Some(d) = _d(&e) {
                if let Ok(l) = serde_json::from_slice::<LicenseInfo>(&d) {
                    if _v(&l) { return l; }
                }
            }
        }
    }
    LicenseInfo::default()
}

#[inline(never)]
pub fn save_license(l: &LicenseInfo) -> Result<(), String> {
    let p = _p();
    if let Some(pr) = p.parent() {
        fs::create_dir_all(pr).map_err(|e| format!("{}", e))?;
    }
    let j = serde_json::to_vec(l).map_err(|e| format!("{}", e))?;
    let e = _e(&j);
    fs::write(&p, e).map_err(|e| format!("{}", e))?;
    Ok(())
}

// ============================================
// ENCRYPTION (Obfuscated names)
// ============================================

#[inline(never)]
fn _k() -> Vec<u8> {
    let m = super::get_machine_id();
    let mut h = DefaultHasher::new();
    m.hash(&mut h);
    let x = h.finish();
    let mut k = Vec::with_capacity(32);
    for i in 0..4u32 {
        let s = x.rotate_left(i * 16);
        k.extend_from_slice(&s.to_le_bytes());
    }
    k
}

#[inline(never)]
fn _e(d: &[u8]) -> Vec<u8> {
    let k = _k();
    let mut r = Vec::with_capacity(d.len() + 8);
    r.extend_from_slice(&(_M1 ^ 0xDEADBEEF).to_le_bytes());
    for (i, b) in d.iter().enumerate() {
        let kb = k[i % k.len()];
        let eb = b ^ kb ^ ((i as u8).wrapping_mul(7));
        r.push(eb);
    }
    let c = _c(&r);
    r.extend_from_slice(&c.to_le_bytes());
    r
}

#[inline(never)]
fn _d(d: &[u8]) -> Option<Vec<u8>> {
    if d.len() < 12 { return None; }
    let h = u32::from_le_bytes([d[0], d[1], d[2], d[3]]);
    if h != (_M1 ^ 0xDEADBEEF) { return None; }
    let sc = u32::from_le_bytes([
        d[d.len() - 4], d[d.len() - 3], d[d.len() - 2], d[d.len() - 1],
    ]);
    let dw = &d[..d.len() - 4];
    let cc = _c(dw);
    if sc != cc { return None; }
    let k = _k();
    let ed = &d[4..d.len() - 4];
    let mut r = Vec::with_capacity(ed.len());
    for (i, b) in ed.iter().enumerate() {
        let kb = k[i % k.len()];
        let db = b ^ kb ^ ((i as u8).wrapping_mul(7));
        r.push(db);
    }
    Some(r)
}

#[inline(never)]
fn _c(d: &[u8]) -> u32 {
    let mut h = DefaultHasher::new();
    d.hash(&mut h);
    (h.finish() & 0xFFFFFFFF) as u32
}

// ============================================
// LICENSE KEY VALIDATION (Obfuscated)
// ============================================

#[inline(never)]
fn _f(k: &str) -> bool {
    if k.len() != 23 { return false; }
    let p: Vec<&str> = k.split('-').collect();
    if p.len() != 4 { return false; }
    p.iter().all(|s| s.len() == 5 && s.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()))
}

#[inline(never)]
pub fn validate_license_key(k: &str) -> Result<bool, String> {
    if !_f(k) { return Err(_err()); }
    let ck: String = k.chars().filter(|c| *c != '-').collect();
    if ck.len() != 20 { return Err(_err()); }
    let px = &ck[..4];
    if !_px(px) { return Err(_err()); }
    let dt = &ck[..16];
    let cs = &ck[16..];
    let cc = _cs(dt);
    if cc != cs { return Err(_err()); }
    let md = &ck[4..16];
    if !_h(md) { return Err(_err()); }
    Ok(true)
}

#[inline(never)]
fn _px(p: &str) -> bool {
    // Obfuscated prefix check
    let v = [
        _s!(83, 67, 80, 82),  // SCPR
        _s!(83, 84, 82, 77),  // STRM
        _s!(67, 76, 73, 80),  // CLIP
        _s!(80, 82, 79, 49),  // PRO1
    ];
    v.iter().any(|x| p == x)
}

#[inline(never)]
fn _cs(d: &str) -> String {
    let mut s: u32 = 0;
    for (i, c) in d.chars().enumerate() {
        let v = c as u32;
        let w = ((i + 1) * 7) as u32;
        s = s.wrapping_add(v.wrapping_mul(w));
    }
    let r = s % 0xFFFF;
    format!("{:04X}", r)
}

#[inline(never)]
fn _h(m: &str) -> bool {
    let mut h = DefaultHasher::new();
    m.hash(&mut h);
    h.finish() != 0
}

// ============================================
// LICENSE INTEGRITY (Obfuscated)
// ============================================

#[inline(never)]
fn _v(l: &LicenseInfo) -> bool {
    if !l.is_pro { return true; }
    let k = match &l.license_key {
        Some(x) => x,
        None => return false,
    };
    if validate_license_key(k).is_err() { return false; }
    let cm = super::get_machine_id();
    if l.machine_id != cm { return false; }
    l.activated_at.is_some()
}

// ============================================
// RUNTIME CHECKS (Multiple entry points)
// ============================================

#[inline(never)]
pub fn runtime_license_check() -> bool {
    let l = load_license();
    let a = l.is_pro;
    let b = l.license_key.is_some();
    let c = _v(&l);
    // Obfuscated boolean combination
    let _ = std::hint::black_box(a);
    let _ = std::hint::black_box(b);
    a && b && c
}

#[inline(never)]
#[allow(dead_code)]
pub fn check_pro_feature(f: &str) -> Result<(), String> {
    let _ = std::hint::black_box(f.len());
    if !runtime_license_check() {
        return Err(format!("{} {}", f, _s!(114, 101, 113, 117, 105, 114, 101, 115, 32, 80, 114, 111))); // "requires Pro"
    }
    Ok(())
}

#[inline(never)]
fn _err() -> String {
    _s!(76, 105, 99, 101, 110, 115, 101, 32, 118, 97, 108, 105, 100, 97, 116, 105, 111, 110, 32, 102, 97, 105, 108, 101, 100) // "License validation failed"
}

// ============================================
// PUBLIC API
// ============================================

#[inline(never)]
pub fn activate_license(k: &str) -> Result<LicenseInfo, String> {
    validate_license_key(k)?;
    let l = LicenseInfo {
        is_pro: true,
        license_key: Some(k.to_string()),
        activated_at: Some(chrono::Utc::now().to_rfc3339()),
        machine_id: super::get_machine_id(),
    };
    save_license(&l)?;
    Ok(l)
}

#[inline(never)]
pub fn deactivate_license() -> Result<(), String> {
    let l = LicenseInfo::default();
    save_license(&l)?;
    Ok(())
}

#[inline(never)]
pub fn requires_pro(f: &str) -> bool {
    matches!(f,
        "chat_detection" | "vertical_crop" | "custom_keywords" |
        "unlimited_clips" | "high_resolution" | "no_watermark" |
        "batch_export" | "fade_effects" | "webm_format"
    )
}

// ============================================
// KEY GENERATION (Test only)
// ============================================

#[cfg(test)]
pub fn generate_valid_key(prefix: &str) -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let middle: String = (0..12).map(|_| chars[rng.random_range(0..chars.len())]).collect();
    let data = format!("{}{}", prefix, middle);
    let checksum = _cs(&data);
    let full = format!("{}{}", data, checksum);
    format!("{}-{}-{}-{}", &full[0..5], &full[5..10], &full[10..15], &full[15..20])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_format() {
        assert!(_f("ABCDE-12345-FGHIJ-67890"));
        assert!(!_f("ABCDE-12345-FGHIJ"));
        assert!(!_f("abcde-12345-fghij-67890"));
    }

    #[test]
    fn test_encryption_roundtrip() {
        let data = b"test license data";
        let encrypted = _e(data);
        let decrypted = _d(&encrypted).unwrap();
        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_generate_and_validate_key() {
        for prefix in ["SCPR", "STRM", "CLIP", "PRO1"] {
            let key = generate_valid_key(prefix);
            println!("Generated key ({}): {}", prefix, key);
            assert!(validate_license_key(&key).is_ok(), "Key should be valid: {}", key);
        }
    }
}
