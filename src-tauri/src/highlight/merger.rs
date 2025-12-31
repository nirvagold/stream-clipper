// Highlight merging utilities

use super::types::Highlight;

/// Merge overlapping highlights
pub fn merge_overlapping_highlights(highlights: &mut Vec<Highlight>, overlap_threshold: f64) {
    if highlights.len() < 2 {
        return;
    }

    // Sort by start time
    highlights.sort_by(|a, b| {
        a.start_secs
            .partial_cmp(&b.start_secs)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut merged: Vec<Highlight> = Vec::new();
    let mut current = highlights[0].clone();

    for highlight in highlights.iter().skip(1) {
        // Calculate overlap percentage
        let overlap = calculate_overlap(&current, highlight);

        if overlap > overlap_threshold {
            // Merge: extend time range, keep higher score
            current.end_secs = current.end_secs.max(highlight.end_secs);
            current.duration_secs = current.end_secs - current.start_secs;

            if highlight.score > current.score {
                current.score = highlight.score;
                current.highlight_type = highlight.highlight_type.clone();
                current.audio_score = highlight.audio_score.or(current.audio_score);
                current.chat_score = highlight.chat_score.or(current.chat_score);
            }

            // Merge reasons
            for reason in &highlight.reasons {
                if !current.reasons.contains(reason) {
                    current.reasons.push(reason.clone());
                }
            }
        } else {
            merged.push(current);
            current = highlight.clone();
        }
    }
    merged.push(current);

    // Re-sort by score and reassign IDs
    merged.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    for (i, h) in merged.iter_mut().enumerate() {
        h.id = (i + 1) as u32;
    }

    *highlights = merged;
}

/// Calculate overlap percentage between two highlights
fn calculate_overlap(h1: &Highlight, h2: &Highlight) -> f64 {
    let overlap_start = h1.start_secs.max(h2.start_secs);
    let overlap_end = h1.end_secs.min(h2.end_secs);

    if overlap_start >= overlap_end {
        return 0.0;
    }

    let overlap_duration = overlap_end - overlap_start;
    let min_duration = h1.duration_secs.min(h2.duration_secs);

    if min_duration > 0.0 {
        overlap_duration / min_duration
    } else {
        0.0
    }
}

/// Apply padding to highlights (for clip export)
pub fn apply_padding(
    highlights: &[Highlight],
    padding_before: f64,
    padding_after: f64,
    video_duration: f64,
) -> Vec<(f64, f64)> {
    highlights
        .iter()
        .map(|h| {
            let start = (h.start_secs - padding_before).max(0.0);
            let end = (h.end_secs + padding_after).min(video_duration);
            (start, end)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::highlight::HighlightType;

    fn create_highlight(id: u32, start: f64, end: f64, score: f32) -> Highlight {
        Highlight {
            id,
            start_secs: start,
            end_secs: end,
            duration_secs: end - start,
            highlight_type: HighlightType::Audio,
            score,
            audio_score: Some(score),
            chat_score: None,
            reasons: vec![],
        }
    }

    #[test]
    fn test_merge_overlapping() {
        let mut highlights = vec![
            create_highlight(1, 0.0, 10.0, 80.0),
            create_highlight(2, 5.0, 12.0, 90.0), // 5-10 overlap = 5s, min_dur=7, overlap=71% > 50%
        ];

        merge_overlapping_highlights(&mut highlights, 0.5);

        assert_eq!(highlights.len(), 1);
        assert_eq!(highlights[0].start_secs, 0.0);
        assert_eq!(highlights[0].end_secs, 12.0);
        assert_eq!(highlights[0].score, 90.0);
    }

    #[test]
    fn test_apply_padding() {
        let highlights = vec![create_highlight(1, 10.0, 20.0, 80.0)];

        let padded = apply_padding(&highlights, 3.0, 2.0, 100.0);

        assert_eq!(padded[0].0, 7.0);
        assert_eq!(padded[0].1, 22.0);
    }
}
