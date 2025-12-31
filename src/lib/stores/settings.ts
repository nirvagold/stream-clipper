// Settings store - from spec 2-design.md Section 2.2
import { writable } from 'svelte/store';
import type { AnalyzeSettings, ExportSettings, Settings } from '$lib/types';

export interface SettingsState {
  // Detection
  audioSensitivity: number;
  audioMinDuration: number;
  audioMergeGap: number;
  chatRateMultiplier: number;
  chatWindowSize: number;
  chatKeywords: string[];
  audioWeight: number;
  chatWeight: number;
  comboBonus: number;
  
  // Export
  outputFolder: string;
  outputFormat: 'Mp4' | 'WebM';
  outputResolution: 'R720p' | 'R1080p' | 'R1440p' | 'R4K' | 'Source';
  paddingBefore: number;
  paddingAfter: number;
  verticalCrop: boolean;
  fadeEffect: boolean;
}

const defaultKeywords = [
  'POG', 'POGGERS', 'LETS GO', 'OMG', 'WTF',
  'CLIP IT', 'GG', 'HOLY', 'INSANE', 'CRAZY'
];

const initialState: SettingsState = {
  audioSensitivity: 1.5,
  audioMinDuration: 2.0,
  audioMergeGap: 3.0,
  chatRateMultiplier: 3.0,
  chatWindowSize: 5.0,
  chatKeywords: [...defaultKeywords],
  audioWeight: 0.6,
  chatWeight: 0.4,
  comboBonus: 1.5,
  outputFolder: '',
  outputFormat: 'Mp4',
  outputResolution: 'R1080p',
  paddingBefore: 3.0,
  paddingAfter: 2.0,
  verticalCrop: false,
  fadeEffect: false,
};

function createSettingsStore() {
  const { subscribe, set, update } = writable<SettingsState>(initialState);

  return {
    subscribe,
    update,
    setOutputFolder: (folder: string) => update(s => ({ ...s, outputFolder: folder })),
    setAudioSensitivity: (value: number) => update(s => ({ ...s, audioSensitivity: value })),
    setAudioMinDuration: (value: number) => update(s => ({ ...s, audioMinDuration: value })),
    setAudioMergeGap: (value: number) => update(s => ({ ...s, audioMergeGap: value })),
    setChatRateMultiplier: (value: number) => update(s => ({ ...s, chatRateMultiplier: value })),
    setAudioWeight: (value: number) => update(s => ({ ...s, audioWeight: value })),
    setChatWeight: (value: number) => update(s => ({ ...s, chatWeight: value })),
    setComboBonus: (value: number) => update(s => ({ ...s, comboBonus: value })),
    setPaddingBefore: (value: number) => update(s => ({ ...s, paddingBefore: value })),
    setPaddingAfter: (value: number) => update(s => ({ ...s, paddingAfter: value })),
    setKeywords: (keywords: string[]) => update(s => ({ ...s, chatKeywords: keywords })),
    setChatKeywords: (keywords: string[]) => update(s => ({ ...s, chatKeywords: keywords })),
    setOutputFormat: (format: 'Mp4' | 'WebM') => update(s => ({ ...s, outputFormat: format })),
    setOutputResolution: (res: SettingsState['outputResolution']) => update(s => ({ ...s, outputResolution: res })),
    setVerticalCrop: (value: boolean) => update(s => ({ ...s, verticalCrop: value })),
    setFadeEffect: (value: boolean) => update(s => ({ ...s, fadeEffect: value })),
    reset: () => set(initialState),
    
    loadSettings: (settings: Settings) => update(s => ({
      ...s,
      audioSensitivity: settings.detection.audio_sensitivity,
      audioMinDuration: settings.detection.audio_min_duration,
      audioMergeGap: settings.detection.audio_merge_gap,
      chatRateMultiplier: settings.detection.chat_rate_multiplier,
      chatWindowSize: settings.detection.chat_window_size,
      chatKeywords: settings.detection.chat_keywords,
      audioWeight: settings.detection.audio_weight,
      chatWeight: settings.detection.chat_weight,
      comboBonus: settings.detection.combo_bonus,
      outputFolder: settings.export.output_folder,
      outputFormat: settings.export.format,
      outputResolution: settings.export.resolution,
      paddingBefore: settings.export.padding_before,
      paddingAfter: settings.export.padding_after,
      verticalCrop: settings.export.vertical_crop,
      fadeEffect: settings.export.fade_effect,
    })),
    
    toAnalyzeSettings: (state: SettingsState): AnalyzeSettings => ({
      audio_sensitivity: state.audioSensitivity,
      audio_min_duration: state.audioMinDuration,
      audio_merge_gap: state.audioMergeGap,
      chat_rate_multiplier: state.chatRateMultiplier,
      chat_window_size: state.chatWindowSize,
      chat_keywords: state.chatKeywords,
      audio_weight: state.audioWeight,
      chat_weight: state.chatWeight,
      combo_bonus: state.comboBonus,
      max_clips: null,
    }),
    
    toExportSettings: (state: SettingsState, addWatermark: boolean): ExportSettings => ({
      output_folder: state.outputFolder,
      format: state.outputFormat,
      resolution: state.outputResolution,
      padding_before: state.paddingBefore,
      padding_after: state.paddingAfter,
      vertical_crop: state.verticalCrop,
      fade_effect: state.fadeEffect,
      add_watermark: addWatermark,
    }),
  };
}

export const settingsStore = createSettingsStore();
