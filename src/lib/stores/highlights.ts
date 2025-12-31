// Highlights store - from spec 2-design.md Section 2.2
import { writable, derived } from 'svelte/store';
import type { Highlight } from '$lib/types';

export interface HighlightsState {
  items: Highlight[];
  selected: Set<number>;
  waveformData: number[];
  isExporting: boolean;
  exportProgress: number;
  exportCurrentClip: number;
  exportTotalClips: number;
}

const initialState: HighlightsState = {
  items: [],
  selected: new Set(),
  waveformData: [],
  isExporting: false,
  exportProgress: 0,
  exportCurrentClip: 0,
  exportTotalClips: 0,
};

function createHighlightsStore() {
  const { subscribe, set, update } = writable<HighlightsState>(initialState);

  return {
    subscribe,
    setHighlights: (highlights: Highlight[], waveform: number[]) => update(s => ({
      ...s,
      items: highlights,
      waveformData: waveform,
      selected: new Set(highlights.slice(0, 3).map(h => h.id)), // Select top 3 by default
    })),
    toggleSelect: (id: number) => update(s => {
      const newSelected = new Set(s.selected);
      if (newSelected.has(id)) {
        newSelected.delete(id);
      } else {
        newSelected.add(id);
      }
      return { ...s, selected: newSelected };
    }),
    selectAll: () => update(s => ({
      ...s,
      selected: new Set(s.items.map(h => h.id)),
    })),
    deselectAll: () => update(s => ({
      ...s,
      selected: new Set(),
    })),
    startExport: (total: number) => update(s => ({
      ...s,
      isExporting: true,
      exportProgress: 0,
      exportCurrentClip: 0,
      exportTotalClips: total,
    })),
    updateExportProgress: (current: number, percent: number) => update(s => ({
      ...s,
      exportCurrentClip: current,
      exportProgress: percent,
    })),
    finishExport: () => update(s => ({
      ...s,
      isExporting: false,
      exportProgress: 100,
    })),
    updateClipTiming: (id: number, startSecs: number, endSecs: number) => update(s => ({
      ...s,
      items: s.items.map(h => 
        h.id === id 
          ? { ...h, start_secs: startSecs, end_secs: endSecs, duration_secs: endSecs - startSecs }
          : h
      ),
    })),
    reset: () => set(initialState),
  };
}

export const highlightsStore = createHighlightsStore();

// Derived store for selected highlights
export const selectedHighlights = derived(
  highlightsStore,
  $store => $store.items.filter(h => $store.selected.has(h.id))
);

export const selectedCount = derived(
  highlightsStore,
  $store => $store.selected.size
);
