// Tauri API wrappers
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
  VideoInfo,
  ChatInfo,
  AnalyzeSettings,
  AnalyzeResult,
  ExportSettings,
  ClipExport,
  ExportResult,
  Settings,
  LicenseInfo,
  AnalyzeProgressEvent,
  ExportProgressEvent,
} from '$lib/types';

// ============================================
// ANALYZE COMMANDS
// ============================================

export async function getVideoInfo(path: string): Promise<VideoInfo> {
  return invoke('get_video_info', { path });
}

export async function getChatInfo(path: string): Promise<ChatInfo> {
  return invoke('get_chat_info', { path });
}

export async function analyzeVideo(
  videoPath: string,
  chatPath: string | null,
  settings: AnalyzeSettings
): Promise<AnalyzeResult> {
  return invoke('analyze_video', { videoPath, chatPath, settings });
}

export async function cancelAnalysis(): Promise<void> {
  return invoke('cancel_analysis');
}

// ============================================
// EXPORT COMMANDS
// ============================================

export async function exportClips(
  videoPath: string,
  clips: ClipExport[],
  settings: ExportSettings
): Promise<ExportResult[]> {
  return invoke('export_clips', { videoPath, clips, settings });
}

export async function previewClip(
  videoPath: string,
  startSecs: number,
  endSecs: number
): Promise<string> {
  return invoke('preview_clip', { videoPath, startSecs, endSecs });
}

export async function cancelExport(): Promise<void> {
  return invoke('cancel_export');
}

export async function openFolder(path: string): Promise<void> {
  return invoke('open_folder', { path });
}

export async function cleanupTempFiles(): Promise<number> {
  return invoke('cleanup_temp_files');
}

// ============================================
// CONFIG COMMANDS
// ============================================

export async function getSettings(): Promise<Settings> {
  return invoke('get_settings');
}

export async function saveSettings(settings: Settings): Promise<void> {
  return invoke('save_settings', { settings });
}

export async function resetSettings(): Promise<Settings> {
  return invoke('reset_settings');
}

export async function getDefaultOutputFolder(): Promise<string> {
  return invoke('get_default_output_folder');
}

export async function pickFolder(): Promise<string | null> {
  return invoke('pick_folder');
}

export async function pickFile(filters: { name: string; extensions: string[] }[]): Promise<string | null> {
  return invoke('pick_file', { filters });
}

// ============================================
// LICENSE COMMANDS
// ============================================

export async function getLicenseStatus(): Promise<LicenseInfo> {
  return invoke('get_license_status');
}

export async function activateLicense(key: string): Promise<LicenseInfo> {
  return invoke('activate_license', { key });
}

export async function deactivateLicense(): Promise<void> {
  return invoke('deactivate_license');
}

// ============================================
// EVENT LISTENERS
// ============================================

export function onAnalyzeProgress(callback: (event: AnalyzeProgressEvent) => void) {
  return listen<AnalyzeProgressEvent>('analyze-progress', (event) => {
    callback(event.payload);
  });
}

export function onExportProgress(callback: (event: ExportProgressEvent) => void) {
  return listen<ExportProgressEvent>('export-progress', (event) => {
    callback(event.payload);
  });
}
