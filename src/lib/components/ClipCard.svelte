<script lang="ts">
  import type { Highlight } from '$lib/types';
  import { highlightsStore, projectStore } from '$lib/stores';
  import { formatDuration, getHighlightEmoji, getScoreClass } from '$lib/utils/format';
  import { ClipPreviewModal } from './modals';

  interface Props {
    highlight: Highlight;
  }

  let { highlight }: Props = $props();
  
  const isSelected = $derived($highlightsStore.selected.has(highlight.id));
  const videoPath = $derived($projectStore.videoPath);

  let showPreview = $state(false);

  function toggleSelect() {
    highlightsStore.toggleSelect(highlight.id);
  }

  function handlePreview(e: MouseEvent) {
    e.stopPropagation();
    showPreview = true;
  }
</script>

<div 
  class="clip-card"
  class:selected={isSelected}
  onclick={toggleSelect}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === 'Enter' && toggleSelect()}
>
  <div class="clip-header">
    <span class="clip-number">#{highlight.id}</span>
    <span class="clip-type">{getHighlightEmoji(highlight.highlight_type)}</span>
  </div>
  
  <div class="clip-score {getScoreClass(highlight.score)}">
    {Math.round(highlight.score)}
  </div>
  
  <div class="clip-time">
    {formatDuration(highlight.start_secs)}
  </div>
  
  <div class="clip-duration">
    {formatDuration(highlight.duration_secs)}
  </div>

  <div class="clip-actions">
    <button 
      class="preview-btn" 
      onclick={handlePreview}
      title="Preview clip"
    >
      👁️
    </button>
    <input 
      type="checkbox" 
      checked={isSelected}
      onclick={(e) => e.stopPropagation()}
      onchange={toggleSelect}
    />
  </div>
</div>

{#if showPreview && videoPath}
  <ClipPreviewModal 
    {highlight}
    {videoPath}
    onClose={() => showPreview = false}
  />
{/if}

<style>
  .clip-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem;
    background: #1f2937;
    border: 2px solid transparent;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
    min-width: 100px;
  }

  .clip-card:hover {
    background: #374151;
  }

  .clip-card.selected {
    border-color: #3b82f6;
    background: #1e3a5f;
  }

  .clip-header {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .clip-number {
    font-size: 0.875rem;
    font-weight: 600;
    color: #d1d5db;
  }

  .clip-type {
    font-size: 1rem;
  }

  .clip-score {
    font-size: 1.5rem;
    font-weight: 700;
  }

  .clip-score.score-high { color: #22c55e; }
  .clip-score.score-medium { color: #eab308; }
  .clip-score.score-low { color: #ef4444; }

  .clip-time {
    font-size: 0.75rem;
    color: #9ca3af;
    font-family: monospace;
  }

  .clip-duration {
    font-size: 0.625rem;
    color: #6b7280;
  }

  .clip-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }

  .preview-btn {
    background: none;
    border: none;
    font-size: 1rem;
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 4px;
    transition: background 0.2s;
  }

  .preview-btn:hover {
    background: #374151;
  }

  .clip-actions input {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }
</style>
