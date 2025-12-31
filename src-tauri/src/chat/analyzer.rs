// Chat activity detection algorithm - from spec 2-design.md Section 3.2

use super::parser::parse_chat;
use super::types::{ChatAnalysisResult, ChatAnalyzeSettings, ChatMessage, ChatSpike, ChatWindow};
use std::collections::HashSet;

/// Analyze chat and detect activity spikes
pub fn analyze_chat(
    chat_path: &str,
    settings: &ChatAnalyzeSettings,
) -> Result<ChatAnalysisResult, String> {
    let (messages, _format) = parse_chat(chat_path)?;

    if messages.is_empty() {
        return Ok(ChatAnalysisResult {
            windows: Vec::new(),
            baseline_rate: 0.0,
            threshold: 0.0,
            spikes: Vec::new(),
        });
    }

    // Get time range
    let start_time = messages.first().map(|m| m.timestamp_secs).unwrap_or(0.0);
    let end_time = messages.last().map(|m| m.timestamp_secs).unwrap_or(0.0);

    // Step 2: Create time windows
    let windows = create_windows(&messages, start_time, end_time, settings);

    if windows.is_empty() {
        return Ok(ChatAnalysisResult {
            windows: Vec::new(),
            baseline_rate: 0.0,
            threshold: 0.0,
            spikes: Vec::new(),
        });
    }

    // Step 4: Calculate baseline
    let rates: Vec<f32> = windows.iter().map(|w| w.message_count as f32).collect();
    let baseline_rate = median(&rates);
    let threshold = baseline_rate * settings.rate_multiplier;

    // Step 5 & 6: Detect and merge spikes
    let spikes = detect_and_merge_spikes(&windows, settings, baseline_rate, threshold);

    Ok(ChatAnalysisResult {
        windows,
        baseline_rate,
        threshold,
        spikes,
    })
}

/// Create time windows and calculate metrics
fn create_windows(
    messages: &[ChatMessage],
    start_time: f64,
    end_time: f64,
    settings: &ChatAnalyzeSettings,
) -> Vec<ChatWindow> {
    let window_size = settings.window_size as f64;
    let mut windows = Vec::new();
    let mut window_start = start_time;

    while window_start < end_time {
        let window_end = window_start + window_size;

        // Get messages in this window
        let window_messages: Vec<&ChatMessage> = messages
            .iter()
            .filter(|m| m.timestamp_secs >= window_start && m.timestamp_secs < window_end)
            .collect();

        let message_count = window_messages.len() as u32;

        // Unique users
        let unique_users: HashSet<&str> = window_messages.iter().map(|m| m.username.as_str()).collect();

        // Keyword matches
        let keyword_matches = count_keyword_matches(&window_messages, &settings.keywords);

        // Emote density
        let emote_count = window_messages.iter().filter(|m| m.has_emote).count();
        let emote_density = if message_count > 0 {
            emote_count as f32 / message_count as f32
        } else {
            0.0
        };

        // Caps ratio
        let caps_ratio = calculate_caps_ratio(&window_messages);

        windows.push(ChatWindow {
            start_secs: window_start,
            end_secs: window_end,
            message_count,
            unique_users: unique_users.len() as u32,
            keyword_matches,
            emote_density,
            caps_ratio,
        });

        window_start = window_end;
    }

    windows
}

/// Count keyword matches in messages
fn count_keyword_matches(messages: &[&ChatMessage], keywords: &[String]) -> u32 {
    let mut count = 0;
    for msg in messages {
        let upper = msg.message.to_uppercase();
        for keyword in keywords {
            if upper.contains(&keyword.to_uppercase()) {
                count += 1;
            }
        }
    }
    count
}

/// Calculate ratio of uppercase characters
fn calculate_caps_ratio(messages: &[&ChatMessage]) -> f32 {
    let mut total_chars = 0;
    let mut upper_chars = 0;

    for msg in messages {
        for c in msg.message.chars() {
            if c.is_alphabetic() {
                total_chars += 1;
                if c.is_uppercase() {
                    upper_chars += 1;
                }
            }
        }
    }

    if total_chars > 0 {
        upper_chars as f32 / total_chars as f32
    } else {
        0.0
    }
}

/// Detect spikes and merge adjacent ones
fn detect_and_merge_spikes(
    windows: &[ChatWindow],
    settings: &ChatAnalyzeSettings,
    baseline: f32,
    threshold: f32,
) -> Vec<ChatSpike> {
    // Find spike windows
    let spike_windows: Vec<&ChatWindow> = windows
        .iter()
        .filter(|w| {
            w.message_count as f32 > threshold
                || w.keyword_matches >= settings.keyword_threshold
                || w.emote_density > settings.emote_threshold
        })
        .collect();

    if spike_windows.is_empty() {
        return Vec::new();
    }

    // Merge adjacent windows
    let mut spikes = Vec::new();
    let mut current_start = spike_windows[0].start_secs;
    let mut current_end = spike_windows[0].end_secs;
    let mut peak_rate = spike_windows[0].message_count as f32;
    let mut total_keywords = spike_windows[0].keyword_matches;

    for window in spike_windows.iter().skip(1) {
        // Check if adjacent (windows are contiguous)
        if (window.start_secs - current_end).abs() < 0.1 {
            // Extend current spike
            current_end = window.end_secs;
            if window.message_count as f32 > peak_rate {
                peak_rate = window.message_count as f32;
            }
            total_keywords += window.keyword_matches;
        } else {
            // Save current spike
            spikes.push(create_chat_spike(
                current_start,
                current_end,
                peak_rate,
                total_keywords,
                baseline,
            ));
            // Start new spike
            current_start = window.start_secs;
            current_end = window.end_secs;
            peak_rate = window.message_count as f32;
            total_keywords = window.keyword_matches;
        }
    }

    // Don't forget the last spike
    spikes.push(create_chat_spike(
        current_start,
        current_end,
        peak_rate,
        total_keywords,
        baseline,
    ));

    spikes
}

/// Create a chat spike with calculated score
fn create_chat_spike(
    start: f64,
    end: f64,
    peak_rate: f32,
    keyword_matches: u32,
    baseline: f32,
) -> ChatSpike {
    // Rate score (how much above baseline)
    let rate_score = if baseline > 0.0 {
        ((peak_rate / baseline - 1.0) * 30.0).clamp(0.0, 70.0)
    } else {
        35.0
    };

    // Keyword score
    let keyword_score = (keyword_matches as f32 * 10.0).min(30.0);

    // Combined score
    let score = (rate_score + keyword_score).min(100.0);

    ChatSpike {
        start_secs: start,
        end_secs: end,
        peak_rate,
        keyword_score,
        score,
    }
}

/// Calculate median
fn median(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mid = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}
