// Project store - from spec 2-design.md Section 2.2
import { writable } from 'svelte/store';
import type { VideoInfo, ChatInfo } from '$lib/types';

export interface ProjectState {
  videoPath: string | null;
  videoInfo: VideoInfo | null;
  chatPath: string | null;
  chatInfo: ChatInfo | null;
  isAnalyzing: boolean;
  analyzeProgress: number;
  analyzeStage: 'idle' | 'extracting' | 'analyzing_audio' | 'analyzing_chat' | 'scoring' | 'complete';
  error: string | null;
}

const initialState: ProjectState = {
  videoPath: null,
  videoInfo: null,
  chatPath: null,
  chatInfo: null,
  isAnalyzing: false,
  analyzeProgress: 0,
  analyzeStage: 'idle',
  error: null,
};

function createProjectStore() {
  const { subscribe, set, update } = writable<ProjectState>(initialState);

  return {
    subscribe,
    setVideo: (path: string, info: VideoInfo) => update(s => ({
      ...s,
      videoPath: path,
      videoInfo: info,
      error: null,
    })),
    setChat: (path: string, info: ChatInfo) => update(s => ({
      ...s,
      chatPath: path,
      chatInfo: info,
    })),
    removeVideo: () => update(s => ({
      ...s,
      videoPath: null,
      videoInfo: null,
    })),
    removeChat: () => update(s => ({
      ...s,
      chatPath: null,
      chatInfo: null,
    })),
    startAnalysis: () => update(s => ({
      ...s,
      isAnalyzing: true,
      analyzeProgress: 0,
      analyzeStage: 'extracting',
      error: null,
    })),
    updateProgress: (stage: ProjectState['analyzeStage'], progress: number) => update(s => ({
      ...s,
      analyzeStage: stage,
      analyzeProgress: progress,
    })),
    finishAnalysis: () => update(s => ({
      ...s,
      isAnalyzing: false,
      analyzeProgress: 100,
      analyzeStage: 'complete',
    })),
    setError: (error: string) => update(s => ({
      ...s,
      isAnalyzing: false,
      error,
    })),
    reset: () => set(initialState),
  };
}

export const projectStore = createProjectStore();
