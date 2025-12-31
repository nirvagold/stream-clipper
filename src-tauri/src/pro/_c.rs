// Pro: Chat analysis - obfuscated
#![allow(non_snake_case)]

use crate::chat::{ChatAnalysisResult, ChatAnalyzeSettings, ChatMessage, ChatSpike, ChatWindow};
use crate::chat::parser::parse_chat;
use crate::license::runtime_license_check;
use std::collections::HashSet;

#[inline(never)]
pub fn pro_analyze_chat(p: &str, s: &ChatAnalyzeSettings) -> Result<ChatAnalysisResult, String> {
    if !runtime_license_check() {
        return Err(_e(0x01));
    }
    let (m, _) = parse_chat(p)?;
    if m.is_empty() { return Ok(_empty()); }
    let (st, et) = (_t(&m, true), _t(&m, false));
    let w = _w(&m, st, et, s);
    if w.is_empty() { return Ok(_empty()); }
    let r: Vec<f32> = w.iter().map(|x| x.message_count as f32).collect();
    let b = _m(&r);
    let th = b * s.rate_multiplier;
    let sp = _ds(&w, s, b, th);
    Ok(ChatAnalysisResult { windows: w, baseline_rate: b, threshold: th, spikes: sp })
}

#[inline(never)] fn _empty() -> ChatAnalysisResult {
    ChatAnalysisResult { windows: vec![], baseline_rate: 0.0, threshold: 0.0, spikes: vec![] }
}

#[inline(never)] fn _t(m: &[ChatMessage], f: bool) -> f64 {
    if f { m.first().map(|x| x.timestamp_secs).unwrap_or(0.0) }
    else { m.last().map(|x| x.timestamp_secs).unwrap_or(0.0) }
}

#[inline(never)] fn _w(ms: &[ChatMessage], st: f64, et: f64, s: &ChatAnalyzeSettings) -> Vec<ChatWindow> {
    let ws = s.window_size as f64;
    let mut r = Vec::new();
    let mut cs = st;
    while cs < et {
        let ce = cs + ws;
        let wm: Vec<&ChatMessage> = ms.iter().filter(|x| x.timestamp_secs >= cs && x.timestamp_secs < ce).collect();
        let mc = wm.len() as u32;
        let uu: HashSet<&str> = wm.iter().map(|x| x.username.as_str()).collect();
        let km = _km(&wm, &s.keywords);
        let ec = wm.iter().filter(|x| x.has_emote).count();
        let ed = if mc > 0 { ec as f32 / mc as f32 } else { 0.0 };
        let cr = _cr(&wm);
        r.push(ChatWindow { start_secs: cs, end_secs: ce, message_count: mc, unique_users: uu.len() as u32, keyword_matches: km, emote_density: ed, caps_ratio: cr });
        cs = ce;
    }
    r
}

#[inline(never)] fn _km(ms: &[&ChatMessage], kw: &[String]) -> u32 {
    let mut c = 0u32;
    for m in ms { let u = m.message.to_uppercase(); for k in kw { if u.contains(&k.to_uppercase()) { c += 1; } } }
    c
}

#[inline(never)] fn _cr(ms: &[&ChatMessage]) -> f32 {
    let (mut t, mut u) = (0u32, 0u32);
    for m in ms { for c in m.message.chars() { if c.is_alphabetic() { t += 1; if c.is_uppercase() { u += 1; } } } }
    if t > 0 { u as f32 / t as f32 } else { 0.0 }
}

#[inline(never)] fn _ds(ws: &[ChatWindow], s: &ChatAnalyzeSettings, b: f32, th: f32) -> Vec<ChatSpike> {
    let sw: Vec<&ChatWindow> = ws.iter().filter(|w| w.message_count as f32 > th || w.keyword_matches >= s.keyword_threshold || w.emote_density > s.emote_threshold).collect();
    if sw.is_empty() { return vec![]; }
    let mut r = Vec::new();
    let (mut cs, mut ce, mut pr, mut tk) = (sw[0].start_secs, sw[0].end_secs, sw[0].message_count as f32, sw[0].keyword_matches);
    for w in sw.iter().skip(1) {
        if (w.start_secs - ce).abs() < 0.1 { ce = w.end_secs; if w.message_count as f32 > pr { pr = w.message_count as f32; } tk += w.keyword_matches; }
        else { r.push(_cs(cs, ce, pr, tk, b)); cs = w.start_secs; ce = w.end_secs; pr = w.message_count as f32; tk = w.keyword_matches; }
    }
    r.push(_cs(cs, ce, pr, tk, b));
    r
}

#[inline(never)] fn _cs(s: f64, e: f64, pr: f32, km: u32, b: f32) -> ChatSpike {
    let rs = if b > 0.0 { ((pr / b - 1.0) * 30.0).clamp(0.0, 70.0) } else { 35.0 };
    let ks = (km as f32 * 10.0).min(30.0);
    let sc = (rs + ks).min(100.0);
    ChatSpike { start_secs: s, end_secs: e, peak_rate: pr, keyword_score: ks, score: sc }
}

#[inline(never)] fn _m(v: &[f32]) -> f32 {
    if v.is_empty() { return 0.0; }
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let m = s.len() / 2;
    if s.len().is_multiple_of(2) { (s[m - 1] + s[m]) / 2.0 } else { s[m] }
}

#[inline(never)] fn _e(c: u8) -> String {
    match c { 0x01 => String::from_utf8_lossy(&[80,114,111,32,114,101,113,117,105,114,101,100]).to_string(), _ => String::new() }
}
