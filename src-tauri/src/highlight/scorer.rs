// Highlight scoring algorithm - from spec 2-design.md Section 3.3

use super::types::{Highlight, HighlightSettings, HighlightType};
use crate::audio::AudioSpike;
use crate::chat::ChatSpike;
use rand::seq::SliceRandom;
use rand::SeedableRng;

/// Score and rank highlights from audio and chat spikes
/// For free tier: randomly selects from all highlights (different each analyze)
/// For pro tier: returns all highlights sorted by score
pub fn score_highlights(
    audio_spikes: &[AudioSpike],
    chat_spikes: &[ChatSpike],
    settings: &HighlightSettings,
) -> Vec<Highlight> {
    // Step 1 & 2: Create unified timeline and detect overlaps
    let mut highlights = Vec::new();
    let mut used_audio: Vec<bool> = vec![false; audio_spikes.len()];
    let mut used_chat: Vec<bool> = vec![false; chat_spikes.len()];

    // Step 3: Find combos (overlapping audio + chat)
    for (ai, audio) in audio_spikes.iter().enumerate() {
        for (ci, chat) in chat_spikes.iter().enumerate() {
            if overlaps(audio.start_secs, audio.end_secs, chat.start_secs, chat.end_secs) {
                // Create combo highlight
                let start = audio.start_secs.min(chat.start_secs);
                let end = audio.end_secs.max(chat.end_secs);
                
                let combined_score = (audio.score * settings.audio_weight
                    + chat.score * settings.chat_weight)
                    * settings.combo_bonus;

                highlights.push(Highlight {
                    id: 0, // Will be assigned later
                    start_secs: start,
                    end_secs: end,
                    duration_secs: end - start,
                    highlight_type: HighlightType::Combo,
                    score: combined_score.min(100.0),
                    audio_score: Some(audio.score),
                    chat_score: Some(chat.score),
                    reasons: vec![
                        format!("🔊 Audio spike: {:.0}% intensity", audio.score),
                        format!("💬 Chat surge: {:.0} messages/window", chat.peak_rate),
                        "⚡ Combo: Audio + Chat happened together (+50% bonus)".to_string(),
                    ],
                });

                used_audio[ai] = true;
                used_chat[ci] = true;
            }
        }
    }

    // Add remaining audio-only highlights
    for (i, audio) in audio_spikes.iter().enumerate() {
        if !used_audio[i] {
            let score = audio.score * settings.audio_weight;
            let mut reasons = vec![format!("🔊 Audio spike: {:.0}% above normal volume", audio.score)];
            
            // Add voice indicator if score is high (indicates VAD detected voice)
            if audio.score > 60.0 {
                reasons.push("🎤 Voice activity detected (streamer reaction)".to_string());
            }
            
            highlights.push(Highlight {
                id: 0,
                start_secs: audio.start_secs,
                end_secs: audio.end_secs,
                duration_secs: audio.end_secs - audio.start_secs,
                highlight_type: HighlightType::Audio,
                score,
                audio_score: Some(audio.score),
                chat_score: None,
                reasons,
            });
        }
    }

    // Add remaining chat-only highlights
    for (i, chat) in chat_spikes.iter().enumerate() {
        if !used_chat[i] {
            let score = chat.score * settings.chat_weight;
            highlights.push(Highlight {
                id: 0,
                start_secs: chat.start_secs,
                end_secs: chat.end_secs,
                duration_secs: chat.end_secs - chat.start_secs,
                highlight_type: HighlightType::Chat,
                score,
                audio_score: None,
                chat_score: Some(chat.score),
                reasons: vec![
                    format!("💬 Chat surge: {:.0} messages/window", chat.peak_rate),
                    format!("🔑 Keyword matches: {:.0} hype words detected", chat.keyword_score),
                ],
            });
        }
    }

    // Step 5: Sort by score descending
    highlights.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    // Step 6: Apply max_clips limit with random selection for free tier
    if let Some(max) = settings.max_clips {
        let max = max as usize;
        if highlights.len() > max {
            // Free tier: randomly select from all highlights
            // This gives free users different clips each time they analyze
            // Use time-based seed so each analyze gives different results
            let seed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(42);
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            
            // Shuffle and take first N
            highlights.shuffle(&mut rng);
            highlights.truncate(max);
            
            // Re-sort by time for better UX (clips appear in video order)
            highlights.sort_by(|a, b| a.start_secs.partial_cmp(&b.start_secs).unwrap_or(std::cmp::Ordering::Equal));
        }
    }

    // Assign sequential IDs
    for (i, h) in highlights.iter_mut().enumerate() {
        h.id = (i + 1) as u32;
    }

    highlights
}

/// Check if two time ranges overlap
fn overlaps(start1: f64, end1: f64, start2: f64, end2: f64) -> bool {
    start1 < end2 && start2 < end1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps() {
        assert!(overlaps(0.0, 10.0, 5.0, 15.0));
        assert!(overlaps(5.0, 15.0, 0.0, 10.0));
        assert!(!overlaps(0.0, 5.0, 10.0, 15.0));
    }

    #[test]
    fn test_score_highlights_audio_only() {
        let audio_spikes = vec![AudioSpike {
            start_secs: 10.0,
            end_secs: 15.0,
            peak_rms: 0.8,
            score: 80.0,
        }];
        let chat_spikes = vec![];
        let settings = HighlightSettings::default();

        let highlights = score_highlights(&audio_spikes, &chat_spikes, &settings);
        
        assert_eq!(highlights.len(), 1);
        assert_eq!(highlights[0].highlight_type, HighlightType::Audio);
    }

    #[test]
    fn test_score_highlights_combo() {
        let audio_spikes = vec![AudioSpike {
            start_secs: 10.0,
            end_secs: 20.0,
            peak_rms: 0.8,
            score: 80.0,
        }];
        let chat_spikes = vec![ChatSpike {
            start_secs: 15.0,
            end_secs: 25.0,
            peak_rate: 50.0,
            keyword_score: 10.0,
            score: 70.0,
        }];
        let settings = HighlightSettings::default();

        let highlights = score_highlights(&audio_spikes, &chat_spikes, &settings);
        
        assert_eq!(highlights.len(), 1);
        assert_eq!(highlights[0].highlight_type, HighlightType::Combo);
    }
}
