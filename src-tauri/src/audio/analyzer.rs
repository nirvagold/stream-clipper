// Audio spike detection algorithm - IMPROVED with VAD (Voice Activity Detection)
// Reduces false positives by detecting human voice vs game sounds

use super::types::{AudioAnalysisResult, AudioAnalyzeSettings, AudioChunk, AudioSpike};
use super::vad::{detect_voice_activity, filter_spikes_by_voice, VadSegment};
use hound::WavReader;
use rayon::prelude::*;
use std::path::Path;

/// Analyze audio file and detect spikes with VAD filtering
pub fn analyze_audio(
    wav_path: &str,
    settings: &AudioAnalyzeSettings,
) -> Result<AudioAnalysisResult, String> {
    let reader = WavReader::open(Path::new(wav_path))
        .map_err(|e| format!("Failed to open WAV file: {}", e))?;

    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let samples: Vec<i16> = reader
        .into_samples::<i16>()
        .filter_map(|s| s.ok())
        .collect();

    if samples.is_empty() {
        return Err("No audio samples found".to_string());
    }

    // Step 1: Run VAD to detect voice segments
    println!("[DEBUG] Running Voice Activity Detection...");
    let vad_segments = detect_voice_activity(&samples, sample_rate, settings.chunk_duration)
        .unwrap_or_else(|e| {
            println!("[WARN] VAD failed: {}, continuing without VAD", e);
            Vec::new()
        });
    
    let voice_segment_count = vad_segments.iter().filter(|s| s.has_voice).count();
    println!("[DEBUG] VAD found {} segments with voice out of {}", voice_segment_count, vad_segments.len());

    // Step 2: Calculate audio chunks
    let samples_per_chunk = (sample_rate as f32 * settings.chunk_duration) as usize;
    let chunks = calculate_chunks_enhanced(&samples, samples_per_chunk, sample_rate, &vad_segments);

    if chunks.is_empty() {
        return Err("No audio chunks generated".to_string());
    }

    // Calculate statistics
    let rms_values: Vec<f32> = chunks.iter().map(|c| c.rms).collect();
    let baseline_rms = median(&rms_values);
    let std_dev = standard_deviation(&rms_values, baseline_rms);

    // Dynamic threshold based on sensitivity
    let percentile = match settings.sensitivity {
        s if s <= 1.0 => 75.0,
        s if s <= 2.0 => 82.0,
        s if s <= 3.0 => 90.0,
        _ => 95.0,
    };
    let mut threshold = percentile_value(&rms_values, percentile);
    
    if threshold <= baseline_rms * 1.15 {
        threshold = baseline_rms * (1.0 + settings.sensitivity * 0.35);
    }

    println!("[DEBUG] Enhanced detection: baseline={:.4}, std_dev={:.4}, threshold={:.4}", 
        baseline_rms, std_dev, threshold);

    // Step 3: Multi-indicator spike detection with VAD
    let spikes = detect_spikes_with_vad(&chunks, settings, baseline_rms, threshold, std_dev, &vad_segments);

    println!("[DEBUG] Detected {} high-quality spikes (VAD filtered)", spikes.len());

    Ok(AudioAnalysisResult {
        chunks,
        baseline_rms,
        std_dev,
        threshold,
        spikes,
    })
}

/// Enhanced chunk calculation with additional metrics and VAD info
fn calculate_chunks_enhanced(
    samples: &[i16], 
    samples_per_chunk: usize, 
    sample_rate: u32,
    vad_segments: &[VadSegment],
) -> Vec<AudioChunk> {
    let chunk_duration = samples_per_chunk as f64 / sample_rate as f64;
    
    let chunks: Vec<AudioChunk> = samples
        .par_chunks(samples_per_chunk)
        .enumerate()
        .map(|(i, chunk)| {
            let start_secs = i as f64 * chunk_duration;
            let end_secs = start_secs + chunk_duration;
            
            // RMS (overall energy)
            let sum_squares: f64 = chunk.iter().map(|&s| (s as f64).powi(2)).sum();
            let rms = (sum_squares / chunk.len() as f64).sqrt() as f32 / 32768.0;
            
            // Peak (maximum amplitude)
            let peak = chunk.iter().map(|&s| (s as i32).abs()).max().unwrap_or(0) as f32 / 32768.0;
            
            AudioChunk {
                start_secs,
                end_secs,
                rms,
                peak,
            }
        })
        .collect();

    // Suppress unused warning - vad_segments used for future enhancement
    let _ = vad_segments;

    chunks
}

/// Enhanced spike detection with VAD (Voice Activity Detection)
fn detect_spikes_with_vad(
    chunks: &[AudioChunk],
    settings: &AudioAnalyzeSettings,
    baseline: f32,
    threshold: f32,
    std_dev: f32,
    vad_segments: &[VadSegment],
) -> Vec<AudioSpike> {
    if chunks.len() < 3 {
        return Vec::new();
    }

    // Calculate delta (change) for each chunk
    let deltas: Vec<f32> = chunks.windows(2)
        .map(|w| (w[1].rms - w[0].rms).abs())
        .collect();
    let avg_delta = deltas.iter().sum::<f32>() / deltas.len() as f32;

    // Find candidate spikes with multiple criteria
    let mut candidates: Vec<(usize, f32, f32)> = Vec::new(); // (index, quality_score, voice_ratio)
    
    for i in 1..chunks.len() - 1 {
        let chunk = &chunks[i];
        let prev = &chunks[i - 1];
        let delta = (chunk.rms - prev.rms).abs();
        
        // Check VAD for this chunk
        let (has_voice, voice_ratio) = filter_spikes_by_voice(
            vad_segments, 
            chunk.start_secs, 
            chunk.end_secs
        );
        
        // Criterion 1: Above threshold
        let above_threshold = chunk.rms > threshold;
        
        // Criterion 2: Significant change from previous (sudden spike)
        let sudden_change = delta > avg_delta * 1.5;
        
        // Criterion 3: Peak-to-RMS ratio (indicates transient/impact sounds)
        let crest_factor = if chunk.rms > 0.001 { chunk.peak / chunk.rms } else { 1.0 };
        let has_transient = crest_factor > 2.0;
        
        // Criterion 4: Sustained energy (not just a blip)
        let sustained = i + 1 < chunks.len() && chunks[i + 1].rms > baseline * 1.2;
        
        // Criterion 5: Significantly above baseline (not just noise)
        let well_above_baseline = chunk.rms > baseline + std_dev * 1.5;
        
        // Criterion 6: HAS VOICE (VAD) - Most important for streamer highlights!
        let voice_detected = has_voice || voice_ratio > 0.2;
        
        // Quality score based on how many criteria are met
        let mut quality = 0.0;
        if above_threshold { quality += 20.0; }
        if sudden_change { quality += 15.0; }
        if has_transient { quality += 10.0; }
        if sustained { quality += 15.0; }
        if well_above_baseline { quality += 10.0; }
        
        // Voice detection is the KEY differentiator - big bonus!
        if voice_detected { 
            quality += 30.0; 
        }
        
        // Count criteria met
        let criteria_met = [above_threshold, sudden_change, has_transient, sustained, well_above_baseline, voice_detected]
            .iter().filter(|&&x| x).count();
        
        // Must meet threshold AND either have voice OR meet 3+ other criteria
        let is_valid = above_threshold && (voice_detected || criteria_met >= 3);
        
        if is_valid {
            candidates.push((i, quality, voice_ratio));
        }
    }

    // Group adjacent candidates into spikes
    let mut spikes = Vec::new();
    if candidates.is_empty() {
        return spikes;
    }

    let mut current_start_idx = candidates[0].0;
    let mut current_end_idx = candidates[0].0;
    let mut current_peak_rms = chunks[candidates[0].0].rms;
    let mut current_quality = candidates[0].1;
    let mut current_voice_ratio = candidates[0].2;

    for &(idx, quality, voice_ratio) in candidates.iter().skip(1) {
        let gap = (chunks[idx].start_secs - chunks[current_end_idx].end_secs) as f32;
        
        if gap <= settings.merge_gap {
            // Extend current spike
            current_end_idx = idx;
            if chunks[idx].rms > current_peak_rms {
                current_peak_rms = chunks[idx].rms;
            }
            current_quality = current_quality.max(quality);
            current_voice_ratio = current_voice_ratio.max(voice_ratio);
        } else {
            // Save current spike if quality is good enough
            let duration = chunks[current_end_idx].end_secs - chunks[current_start_idx].start_secs;
            if duration >= settings.min_duration as f64 && current_quality >= 40.0 {
                spikes.push(create_spike_with_vad(
                    chunks[current_start_idx].start_secs,
                    chunks[current_end_idx].end_secs,
                    current_peak_rms,
                    baseline,
                    current_quality,
                    current_voice_ratio,
                ));
            }
            // Start new spike
            current_start_idx = idx;
            current_end_idx = idx;
            current_peak_rms = chunks[idx].rms;
            current_quality = quality;
            current_voice_ratio = voice_ratio;
        }
    }

    // Don't forget the last spike
    let duration = chunks[current_end_idx].end_secs - chunks[current_start_idx].start_secs;
    if duration >= settings.min_duration as f64 && current_quality >= 40.0 {
        spikes.push(create_spike_with_vad(
            chunks[current_start_idx].start_secs,
            chunks[current_end_idx].end_secs,
            current_peak_rms,
            baseline,
            current_quality,
            current_voice_ratio,
        ));
    }

    // Sort by score and limit to top highlights
    spikes.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    
    spikes
}

/// Create spike with VAD-enhanced scoring
fn create_spike_with_vad(start: f64, end: f64, peak_rms: f32, baseline: f32, quality: f32, voice_ratio: f32) -> AudioSpike {
    // Combine intensity score with quality score
    let intensity_score = if baseline > 0.0 {
        ((peak_rms / baseline - 1.0) * 30.0).clamp(0.0, 40.0)
    } else {
        20.0
    };
    
    // Quality contributes to final score
    let quality_bonus = (quality - 40.0).max(0.0) * 0.6;
    
    // Voice ratio bonus - clips with voice are more valuable
    let voice_bonus = voice_ratio * 20.0;
    
    let score = (intensity_score + quality_bonus + voice_bonus).clamp(0.0, 100.0);

    AudioSpike {
        start_secs: start,
        end_secs: end,
        peak_rms,
        score,
    }
}

fn percentile_value(values: &[f32], percentile: f32) -> f32 {
    if values.is_empty() { return 0.0; }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let index = ((percentile / 100.0) * (sorted.len() - 1) as f32) as usize;
    sorted[index.min(sorted.len() - 1)]
}

fn median(values: &[f32]) -> f32 {
    if values.is_empty() { return 0.0; }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) { (sorted[mid - 1] + sorted[mid]) / 2.0 } else { sorted[mid] }
}

fn standard_deviation(values: &[f32], mean: f32) -> f32 {
    if values.is_empty() { return 0.0; }
    let variance: f32 = values.iter().map(|&v| (v - mean).powi(2)).sum::<f32>() / values.len() as f32;
    variance.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        assert_eq!(median(&[1.0, 2.0, 3.0]), 2.0);
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
    }
}
