// Voice Activity Detection (VAD) module
// Detects human voice vs game sounds/music

use webrtc_vad::{SampleRate, Vad, VadMode};

/// VAD analysis result for a time segment
#[derive(Debug, Clone)]
pub struct VadSegment {
    pub start_secs: f64,
    pub end_secs: f64,
    pub voice_ratio: f32,  // 0.0 - 1.0, percentage of frames with voice
    pub has_voice: bool,
}

/// Analyze audio samples for voice activity
/// Returns segments where human voice is detected
pub fn detect_voice_activity(
    samples: &[i16],
    sample_rate: u32,
    chunk_duration_secs: f32,
) -> Result<Vec<VadSegment>, String> {
    // WebRTC VAD only supports specific sample rates
    let vad_sample_rate = match sample_rate {
        8000 => SampleRate::Rate8kHz,
        16000 => SampleRate::Rate16kHz,
        32000 => SampleRate::Rate32kHz,
        48000 => SampleRate::Rate48kHz,
        _ => {
            // Resample to 16kHz if needed (most common for voice)
            return detect_voice_with_resample(samples, sample_rate, chunk_duration_secs);
        }
    };

    let mut vad = Vad::new_with_rate_and_mode(vad_sample_rate, VadMode::Aggressive);

    // VAD frame size must be 10, 20, or 30 ms
    let frame_ms = 30;
    let frame_samples = (sample_rate as usize * frame_ms) / 1000;
    let samples_per_chunk = (sample_rate as f32 * chunk_duration_secs) as usize;

    let mut segments = Vec::new();
    let mut chunk_start = 0usize;

    while chunk_start < samples.len() {
        let chunk_end = (chunk_start + samples_per_chunk).min(samples.len());
        let chunk = &samples[chunk_start..chunk_end];

        // Count voice frames in this chunk
        let mut voice_frames = 0;
        let mut total_frames = 0;

        let mut frame_start = 0;
        while frame_start + frame_samples <= chunk.len() {
            let frame = &chunk[frame_start..frame_start + frame_samples];
            
            if let Ok(is_voice) = vad.is_voice_segment(frame) {
                total_frames += 1;
                if is_voice {
                    voice_frames += 1;
                }
            }
            
            frame_start += frame_samples;
        }

        let voice_ratio = if total_frames > 0 {
            voice_frames as f32 / total_frames as f32
        } else {
            0.0
        };

        let start_secs = chunk_start as f64 / sample_rate as f64;
        let end_secs = chunk_end as f64 / sample_rate as f64;

        segments.push(VadSegment {
            start_secs,
            end_secs,
            voice_ratio,
            has_voice: voice_ratio > 0.3, // At least 30% of frames have voice
        });

        chunk_start = chunk_end;
    }

    Ok(segments)
}

/// Resample audio to 16kHz for VAD processing
fn detect_voice_with_resample(
    samples: &[i16],
    original_rate: u32,
    chunk_duration_secs: f32,
) -> Result<Vec<VadSegment>, String> {
    let target_rate = 16000u32;
    let ratio = original_rate as f64 / target_rate as f64;

    // Simple linear interpolation resampling
    let new_len = (samples.len() as f64 / ratio) as usize;
    let mut resampled = Vec::with_capacity(new_len);

    for i in 0..new_len {
        let src_idx = i as f64 * ratio;
        let idx0 = src_idx.floor() as usize;
        let idx1 = (idx0 + 1).min(samples.len() - 1);
        let frac = src_idx - idx0 as f64;

        let sample = samples[idx0] as f64 * (1.0 - frac) + samples[idx1] as f64 * frac;
        resampled.push(sample as i16);
    }

    // Now process with 16kHz
    let mut vad = Vad::new_with_rate_and_mode(SampleRate::Rate16kHz, VadMode::Aggressive);

    let frame_ms = 30;
    let frame_samples = (target_rate as usize * frame_ms) / 1000; // 480 samples
    let samples_per_chunk = (target_rate as f32 * chunk_duration_secs) as usize;

    let mut segments = Vec::new();
    let mut chunk_start = 0usize;

    while chunk_start < resampled.len() {
        let chunk_end = (chunk_start + samples_per_chunk).min(resampled.len());
        let chunk = &resampled[chunk_start..chunk_end];

        let mut voice_frames = 0;
        let mut total_frames = 0;

        let mut frame_start = 0;
        while frame_start + frame_samples <= chunk.len() {
            let frame = &chunk[frame_start..frame_start + frame_samples];
            
            if let Ok(is_voice) = vad.is_voice_segment(frame) {
                total_frames += 1;
                if is_voice {
                    voice_frames += 1;
                }
            }
            
            frame_start += frame_samples;
        }

        let voice_ratio = if total_frames > 0 {
            voice_frames as f32 / total_frames as f32
        } else {
            0.0
        };

        // Convert back to original time scale
        let start_secs = (chunk_start as f64 * ratio) / original_rate as f64;
        let end_secs = (chunk_end as f64 * ratio) / original_rate as f64;

        segments.push(VadSegment {
            start_secs,
            end_secs,
            voice_ratio,
            has_voice: voice_ratio > 0.3,
        });

        chunk_start = chunk_end;
    }

    Ok(segments)
}

/// Merge VAD results with audio spikes to filter out non-voice spikes
pub fn filter_spikes_by_voice(
    vad_segments: &[VadSegment],
    spike_start: f64,
    spike_end: f64,
) -> (bool, f32) {
    // Find VAD segments that overlap with this spike
    let mut total_voice_ratio = 0.0;
    let mut overlap_count = 0;

    for seg in vad_segments {
        if seg.start_secs < spike_end && seg.end_secs > spike_start {
            total_voice_ratio += seg.voice_ratio;
            overlap_count += 1;
        }
    }

    if overlap_count == 0 {
        return (false, 0.0);
    }

    let avg_voice_ratio = total_voice_ratio / overlap_count as f32;
    let has_significant_voice = avg_voice_ratio > 0.25; // At least 25% voice

    (has_significant_voice, avg_voice_ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vad_segment_creation() {
        let seg = VadSegment {
            start_secs: 0.0,
            end_secs: 0.5,
            voice_ratio: 0.5,
            has_voice: true,
        };
        assert!(seg.has_voice);
    }
}
