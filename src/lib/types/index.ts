// TypeScript types - mirrors Rust types for IPC
// From spec 2-design.md Section 2.2

// ============================================
// VIDEO & PROJECT
// ============================================

export interface VideoInfo {
  path: string;
  filename: string;
  duration_secs: number;
  width: number;
  height: number;
  fps: number;
  codec: string;
  file_size_bytes: number;
}

export interface ChatInfo {
  path: string;
  format: ChatFormat;
  total_messages: number;
  duration_secs: number;
  avg_rate_per_min: number;
}

export type ChatFormat = 'TwitchJson' | 'YouTubeJson' | 'GenericTxt' | 'Unknown';

// ============================================
// HIGHLIGHTS
// ============================================

export type HighlightType = 'Audio' | 'Chat' | 'Combo';

export interface Highlight {
  id: number;
  start_secs: number;
  end_secs: number;
  duration_secs: number;
  highlight_type: HighlightType;
  score: number;
  audio_score: number | null;
  chat_score: number | null;
  reasons: string[];
}

export interface AnalyzeResult {
  highlights: Highlight[];
  waveform_data: number[];
  total_duration_secs: number;
}

// ============================================
// SETTINGS
// ============================================

export interface AnalyzeSettings {
  audio_sensitivity: number;
  audio_min_duration: number;
  audio_merge_gap: number;
  chat_rate_multiplier: number;
  chat_window_size: number;
  chat_keywords: string[];
  audio_weight: number;
  chat_weight: number;
  combo_bonus: number;
  max_clips: number | null;
}

export type OutputFormat = 'Mp4' | 'WebM';
export type OutputResolution = 'R720p' | 'R1080p' | 'R1440p' | 'R4K' | 'Source';

export interface ExportSettings {
  output_folder: string;
  format: OutputFormat;
  resolution: OutputResolution;
  padding_before: number;
  padding_after: number;
  vertical_crop: boolean;
  fade_effect: boolean;
  add_watermark: boolean;
}

export interface Settings {
  version: number;
  detection: AnalyzeSettings;
  export: ExportSettings;
}

// ============================================
// EXPORT
// ============================================

export interface ClipExport {
  highlight_id: number;
  start_secs: number;
  end_secs: number;
}

export interface ExportResult {
  highlight_id: number;
  output_path: string;
  success: boolean;
  error: string | null;
}

// ============================================
// LICENSE
// ============================================

export interface LicenseInfo {
  is_pro: boolean;
  license_key: string | null;
  activated_at: string | null;
  machine_id: string;
}

// ============================================
// EVENTS
// ============================================

export interface AnalyzeProgressEvent {
  stage: 'extracting' | 'analyzing_audio' | 'analyzing_chat' | 'scoring' | 'complete';
  progress: number;
  message: string;
}

export interface ExportProgressEvent {
  current: number;
  total: number;
  percent: number;
  message: string;
}
