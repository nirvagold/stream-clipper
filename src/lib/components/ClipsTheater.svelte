<script lang="ts">
  import type { Highlight } from '$lib/types';
  import { highlightsStore, projectStore } from '$lib/stores';
  import { previewClip } from '$lib/utils/tauri';
  import { formatDuration, getHighlightEmoji, getScoreClass, getHighlightColor } from '$lib/utils/format';
  import { convertFileSrc } from '@tauri-apps/api/core';

  const highlights = $derived($highlightsStore.items);
  const selectedIds = $derived($highlightsStore.selected);
  const videoPath = $derived($projectStore.videoPath);
  const videoDuration = $derived($projectStore.videoInfo?.duration_secs ?? 0);

  const sortedHighlights = $derived(
    [...highlights].sort((a, b) => a.start_secs - b.start_secs)
  );

  let activeClipId = $state<number | null>(null);
  let previewSrc = $state<string | null>(null);
  let isLoadingPreview = $state(false);
  let previewError = $state<string | null>(null);
  let videoElement = $state<HTMLVideoElement | null>(null);

  // Editing state (internal values in seconds)
  let isEditing = $state(false);
  let editStart = $state(0);
  let editDuration = $state(0);
  let editEnd = $state(0);
  let hasUnsavedChanges = $state(false);

  // Input display values (MM:SS for start/end, seconds for duration)
  let startMinutes = $state(0);
  let startSeconds = $state(0);
  let endMinutes = $state(0);
  let endSeconds = $state(0);
  let durationInput = $state(0);

  const MIN_DURATION = 0.5;

  const activeClip = $derived(
    activeClipId !== null ? highlights.find(h => h.id === activeClipId) : null
  );

  // Convert seconds to MM:SS components
  function secsToMinSec(secs: number): { min: number; sec: number } {
    const min = Math.floor(secs / 60);
    const sec = Math.round((secs % 60) * 10) / 10;
    return { min, sec };
  }

  // Convert MM:SS to seconds
  function minSecToSecs(min: number, sec: number): number {
    return min * 60 + sec;
  }

  // Sync display values from internal seconds
  function syncDisplayFromInternal() {
    const start = secsToMinSec(editStart);
    startMinutes = start.min;
    startSeconds = start.sec;
    
    const end = secsToMinSec(editEnd);
    endMinutes = end.min;
    endSeconds = end.sec;
    
    durationInput = Math.round(editDuration * 10) / 10;
  }

  $effect(() => {
    if (sortedHighlights.length > 0 && activeClipId === null) {
      activeClipId = sortedHighlights[0].id;
    }
  });

  $effect(() => {
    if (activeClipId !== null && videoPath && !isEditing) {
      loadPreview(activeClipId);
    }
  });

  $effect(() => {
    if (activeClip && !isEditing) {
      editStart = activeClip.start_secs;
      editEnd = activeClip.end_secs;
      editDuration = activeClip.duration_secs;
      syncDisplayFromInternal();
    }
  });

  async function loadPreview(clipId: number, customStart?: number, customEnd?: number) {
    const clip = highlights.find(h => h.id === clipId);
    if (!clip || !videoPath) return;

    const startSecs = customStart ?? clip.start_secs;
    const endSecs = customEnd ?? clip.end_secs;

    isLoadingPreview = true;
    previewError = null;
    previewSrc = null;

    try {
      const path = await previewClip(videoPath, startSecs, endSecs);
      if (activeClipId === clipId) {
        previewSrc = convertFileSrc(path);
      }
    } catch (e) {
      if (activeClipId === clipId) {
        previewError = e instanceof Error ? e.message : String(e);
      }
    } finally {
      if (activeClipId === clipId) {
        isLoadingPreview = false;
      }
    }
  }

  function selectClip(id: number) {
    if (hasUnsavedChanges) {
      if (!confirm('You have unsaved changes. Discard them?')) return;
    }
    isEditing = false;
    hasUnsavedChanges = false;
    activeClipId = id;
  }

  function startEditing() {
    if (!activeClip) return;
    editStart = activeClip.start_secs;
    editEnd = activeClip.end_secs;
    editDuration = activeClip.duration_secs;
    syncDisplayFromInternal();
    isEditing = true;
    hasUnsavedChanges = false;
  }

  function cancelEditing() {
    if (!activeClip) return;
    editStart = activeClip.start_secs;
    editEnd = activeClip.end_secs;
    editDuration = activeClip.duration_secs;
    syncDisplayFromInternal();
    isEditing = false;
    hasUnsavedChanges = false;
    loadPreview(activeClip.id);
  }

  function saveChanges() {
    if (!activeClip) return;
    highlightsStore.updateClipTiming(activeClip.id, editStart, editEnd);
    isEditing = false;
    hasUnsavedChanges = false;
  }

  async function previewChanges() {
    if (!activeClip || !videoPath) return;
    await loadPreview(activeClip.id, editStart, editEnd);
  }

  // Handle Start change (MM:SS) → Duration changes, End stays
  function handleStartChange() {
    let newStart = minSecToSecs(startMinutes, startSeconds);
    // Clamp: 0 ≤ start ≤ end - MIN_DURATION
    newStart = Math.max(0, Math.min(newStart, editEnd - MIN_DURATION));
    editStart = Math.round(newStart * 10) / 10;
    editDuration = Math.round((editEnd - editStart) * 10) / 10;
    syncDisplayFromInternal();
    hasUnsavedChanges = true;
  }

  // Handle Duration change (seconds) → End changes, Start stays
  function handleDurationChange() {
    let newDuration = durationInput;
    // Clamp: MIN_DURATION ≤ duration ≤ videoDuration - start
    const maxDuration = videoDuration - editStart;
    newDuration = Math.max(MIN_DURATION, Math.min(newDuration, maxDuration));
    editDuration = Math.round(newDuration * 10) / 10;
    editEnd = Math.round((editStart + editDuration) * 10) / 10;
    syncDisplayFromInternal();
    hasUnsavedChanges = true;
  }

  // Handle End change (MM:SS) → Duration changes, Start stays
  function handleEndChange() {
    let newEnd = minSecToSecs(endMinutes, endSeconds);
    // Clamp: start + MIN_DURATION ≤ end ≤ videoDuration
    newEnd = Math.max(editStart + MIN_DURATION, Math.min(newEnd, videoDuration));
    editEnd = Math.round(newEnd * 10) / 10;
    editDuration = Math.round((editEnd - editStart) * 10) / 10;
    syncDisplayFromInternal();
    hasUnsavedChanges = true;
  }

  function selectAll() { highlightsStore.selectAll(); }
  function deselectAll() { highlightsStore.deselectAll(); }

  const selectedCount = $derived(selectedIds.size);
</script>

<div class="clips-theater">
  {#if activeClip}
    <div class="theater-section">
      <div class="theater-container">
        <div class="video-wrapper">
          {#if isLoadingPreview}
            <div class="loading-state">
              <div class="spinner"></div>
              <p>Generating preview...</p>
            </div>
          {:else if previewError}
            <div class="error-state">
              <p>❌ Failed to load preview</p>
              <p class="error-detail">{previewError}</p>
            </div>
          {:else if previewSrc}
            <video 
              bind:this={videoElement} 
              src={previewSrc} 
              controls 
              autoplay 
              controlslist="nodownload noplaybackrate"
              disablepictureinpicture
              oncontextmenu={(e) => e.preventDefault()}
              class="theater-video"
            >
              <track kind="captions" />
            </video>
          {:else}
            <div class="placeholder-state">
              <p>🎬 Select a clip to preview</p>
            </div>
          {/if}
        </div>
      </div>

      <div class="clip-details">
        <div class="clip-header">
          <div class="clip-title">
            <span class="clip-emoji">{getHighlightEmoji(activeClip.highlight_type)}</span>
            <span class="clip-name">Clip #{activeClip.id}</span>
            <span class="clip-type-badge" style="background: {getHighlightColor(activeClip.highlight_type)}">
              {activeClip.highlight_type}
            </span>
          </div>
          <div class="clip-score {getScoreClass(activeClip.score)}">
            Score: {Math.round(activeClip.score)}
          </div>
        </div>

        <!-- Score explanation -->
        {#if activeClip.reasons && activeClip.reasons.length > 0}
          <div class="score-reasons">
            <span class="reasons-label">Why this clip?</span>
            <ul class="reasons-list">
              {#each activeClip.reasons as reason}
                <li>{reason}</li>
              {/each}
            </ul>
          </div>
        {/if}

        {#if isEditing}
          <div class="timing-editor">
            <div class="timing-fields">
              <!-- Start Time (MM:SS) -->
              <div class="timing-field">
                <span class="field-label">Start Time</span>
                <div class="time-input-group">
                  <input
                    type="number"
                    min="0"
                    max="999"
                    bind:value={startMinutes}
                    onchange={handleStartChange}
                    class="time-input minutes"
                    aria-label="Start minutes"
                  />
                  <span class="time-separator">:</span>
                  <input
                    type="number"
                    min="0"
                    max="59.9"
                    step="0.1"
                    bind:value={startSeconds}
                    onchange={handleStartChange}
                    class="time-input seconds"
                    aria-label="Start seconds"
                  />
                </div>
                <span class="time-hint">MM : SS</span>
              </div>

              <!-- Duration (seconds) -->
              <div class="timing-field duration-field">
                <span class="field-label">Duration</span>
                <div class="duration-input-group">
                  <input
                    type="number"
                    min={MIN_DURATION}
                    step="0.1"
                    bind:value={durationInput}
                    onchange={handleDurationChange}
                    class="duration-input"
                    aria-label="Duration in seconds"
                  />
                  <span class="duration-unit">sec</span>
                </div>
                <span class="time-hint">{formatDuration(editDuration)}</span>
              </div>

              <!-- End Time (MM:SS) -->
              <div class="timing-field">
                <span class="field-label">End Time</span>
                <div class="time-input-group">
                  <input
                    type="number"
                    min="0"
                    max="999"
                    bind:value={endMinutes}
                    onchange={handleEndChange}
                    class="time-input minutes"
                    aria-label="End minutes"
                  />
                  <span class="time-separator">:</span>
                  <input
                    type="number"
                    min="0"
                    max="59.9"
                    step="0.1"
                    bind:value={endSeconds}
                    onchange={handleEndChange}
                    class="time-input seconds"
                    aria-label="End seconds"
                  />
                </div>
                <span class="time-hint">MM : SS</span>
              </div>
            </div>

            <div class="editor-actions">
              <button class="btn btn-preview" onclick={previewChanges} disabled={isLoadingPreview}>
                {isLoadingPreview ? '⏳ Loading...' : '▶ Preview'}
              </button>
              <button class="btn btn-save" onclick={saveChanges}>✓ Save</button>
              <button class="btn btn-cancel" onclick={cancelEditing}>✕ Cancel</button>
            </div>
          </div>
        {:else}
          <div class="clip-meta">
            <div class="meta-item">
              <span class="meta-label">Start</span>
              <span class="meta-value">{formatDuration(activeClip.start_secs)}</span>
            </div>
            <div class="meta-item">
              <span class="meta-label">Duration</span>
              <span class="meta-value">{activeClip.duration_secs.toFixed(1)}s</span>
            </div>
            <div class="meta-item">
              <span class="meta-label">End</span>
              <span class="meta-value">{formatDuration(activeClip.end_secs)}</span>
            </div>
            {#if activeClip.audio_score !== null}
              <div class="meta-item">
                <span class="meta-label">Audio</span>
                <span class="meta-value">{Math.round(activeClip.audio_score)}</span>
              </div>
            {/if}
            {#if activeClip.chat_score !== null}
              <div class="meta-item">
                <span class="meta-label">Chat</span>
                <span class="meta-value">{Math.round(activeClip.chat_score)}</span>
              </div>
            {/if}
          </div>

          <div class="clip-actions">
            <button class="btn btn-edit" onclick={startEditing}>✏️ Edit Timing</button>
            <label class="export-checkbox">
              <input
                type="checkbox"
                checked={selectedIds.has(activeClip.id)}
                onchange={() => highlightsStore.toggleSelect(activeClip.id)}
              />
              <span>Include in export</span>
            </label>
          </div>
        {/if}
      </div>
    </div>
  {/if}

  <div class="thumbnails-section">
    <div class="thumbnails-header">
      <h3>🎬 All Clips ({sortedHighlights.length})</h3>
      <div class="thumbnails-actions">
        <button class="action-btn" onclick={selectAll}>Select All</button>
        <button class="action-btn" onclick={deselectAll}>Deselect All</button>
        <span class="selected-count">{selectedCount} selected for export</span>
      </div>
    </div>

    <div class="thumbnails-grid">
      {#each sortedHighlights as clip (clip.id)}
        <button
          class="thumbnail-card"
          class:active={activeClipId === clip.id}
          class:selected={selectedIds.has(clip.id)}
          onclick={() => selectClip(clip.id)}
        >
          <div class="thumbnail-preview">
            <span class="thumbnail-emoji">{getHighlightEmoji(clip.highlight_type)}</span>
            <span class="thumbnail-time">{formatDuration(clip.start_secs)}</span>
          </div>
          <div class="thumbnail-info">
            <span class="thumbnail-id">#{clip.id}</span>
            <span class="thumbnail-score {getScoreClass(clip.score)}">{Math.round(clip.score)}</span>
          </div>
          <div class="thumbnail-checkbox">
            <input
              type="checkbox"
              checked={selectedIds.has(clip.id)}
              onchange={() => highlightsStore.toggleSelect(clip.id)}
              onclick={(e: MouseEvent) => e.stopPropagation()}
              aria-label="Select clip {clip.id} for export"
            />
          </div>
          {#if activeClipId === clip.id}
            <div class="now-playing">▶ Now Playing</div>
          {/if}
        </button>
      {/each}
    </div>
  </div>
</div>

<style>
  .clips-theater { display: flex; flex-direction: column; gap: 1.5rem; }

  .theater-section { background: #1f2937; border-radius: 12px; overflow: hidden; }
  .theater-container { background: #000; aspect-ratio: 16 / 9; max-height: 450px; }
  .video-wrapper { width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; }
  .theater-video { width: 100%; height: 100%; object-fit: contain; }

  .loading-state, .error-state, .placeholder-state {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    gap: 1rem; color: #9ca3af; text-align: center; padding: 2rem;
  }
  .spinner { width: 40px; height: 40px; border: 3px solid #374151; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
  .error-state { color: #ef4444; }
  .error-detail { font-size: 0.75rem; color: #9ca3af; }

  .clip-details { padding: 1.25rem; border-top: 1px solid #374151; }
  .clip-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem; }
  .clip-title { display: flex; align-items: center; gap: 0.5rem; }
  .clip-emoji { font-size: 1.5rem; }
  .clip-name { font-size: 1.125rem; font-weight: 600; color: #f3f4f6; }
  .clip-type-badge { font-size: 0.625rem; padding: 0.25rem 0.5rem; border-radius: 9999px; color: white; font-weight: 600; text-transform: uppercase; }
  .clip-score { font-size: 1.25rem; font-weight: 700; }
  .clip-score.score-high { color: #22c55e; }
  .clip-score.score-medium { color: #eab308; }
  .clip-score.score-low { color: #ef4444; }

  /* Score Reasons */
  .score-reasons {
    background: #111827; border-radius: 6px; padding: 0.75rem 1rem;
    margin-bottom: 1rem; border-left: 3px solid #3b82f6;
  }
  .reasons-label { font-size: 0.75rem; color: #9ca3af; text-transform: uppercase; font-weight: 600; display: block; margin-bottom: 0.5rem; }
  .reasons-list { margin: 0; padding-left: 1.25rem; }
  .reasons-list li { font-size: 0.8rem; color: #d1d5db; margin-bottom: 0.25rem; }
  .reasons-list li:last-child { margin-bottom: 0; }

  /* Timing Editor */
  .timing-editor { background: #111827; border-radius: 8px; padding: 1rem; }

  .timing-fields { display: flex; flex-wrap: wrap; gap: 2rem; margin-bottom: 1rem; align-items: flex-start; }
  
  .timing-field { display: flex; flex-direction: column; gap: 0.5rem; }
  .timing-field .field-label { font-size: 0.75rem; color: #9ca3af; text-transform: uppercase; font-weight: 600; }
  
  .time-input-group { display: flex; align-items: center; gap: 0.25rem; }
  .time-separator { color: #6b7280; font-size: 1.25rem; font-weight: bold; }
  
  .time-input {
    padding: 0.5rem; background: #1f2937; border: 1px solid #374151;
    border-radius: 6px; color: #f3f4f6; font-family: monospace; font-size: 1rem; text-align: center;
  }
  .time-input.minutes { width: 60px; }
  .time-input.seconds { width: 70px; }
  .time-input:focus { outline: none; border-color: #3b82f6; }
  
  .time-hint { font-size: 0.7rem; color: #6b7280; text-align: center; }

  .duration-field { display: flex; flex-direction: column; gap: 0.5rem; }
  .duration-input-group { display: flex; align-items: center; gap: 0.5rem; }
  .duration-input {
    width: 80px; padding: 0.5rem; background: #1f2937; border: 1px solid #374151;
    border-radius: 6px; color: #f3f4f6; font-family: monospace; font-size: 1rem; text-align: center;
  }
  .duration-input:focus { outline: none; border-color: #3b82f6; }
  .duration-unit { color: #9ca3af; font-size: 0.875rem; }

  .editor-actions { display: flex; gap: 0.75rem; flex-wrap: wrap; }

  .btn { padding: 0.5rem 1rem; border: none; border-radius: 6px; font-size: 0.875rem; font-weight: 500; cursor: pointer; transition: all 0.2s; }
  .btn-preview { background: #3b82f6; color: white; }
  .btn-preview:hover:not(:disabled) { background: #2563eb; }
  .btn-preview:disabled { opacity: 0.5; cursor: not-allowed; }
  .btn-save { background: #22c55e; color: white; }
  .btn-save:hover { background: #16a34a; }
  .btn-cancel { background: #374151; color: #d1d5db; }
  .btn-cancel:hover { background: #4b5563; }
  .btn-edit { background: #374151; color: #d1d5db; }
  .btn-edit:hover { background: #4b5563; }

  .clip-meta { display: flex; flex-wrap: wrap; gap: 1.5rem; margin-bottom: 1rem; }
  .meta-item { display: flex; flex-direction: column; gap: 0.25rem; }
  .meta-label { font-size: 0.75rem; color: #6b7280; text-transform: uppercase; }
  .meta-value { font-size: 0.875rem; color: #f3f4f6; font-family: monospace; }

  .clip-actions { display: flex; gap: 1rem; align-items: center; }
  .export-checkbox { display: flex; align-items: center; gap: 0.5rem; cursor: pointer; font-size: 0.875rem; color: #d1d5db; }
  .export-checkbox input { width: 18px; height: 18px; }

  /* Thumbnails */
  .thumbnails-section { background: #1f2937; border-radius: 12px; padding: 1.25rem; }
  .thumbnails-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem; flex-wrap: wrap; gap: 0.75rem; }
  .thumbnails-header h3 { font-size: 1rem; font-weight: 600; color: #f3f4f6; margin: 0; }
  .thumbnails-actions { display: flex; align-items: center; gap: 0.75rem; }
  .action-btn { padding: 0.375rem 0.75rem; background: #374151; border: none; border-radius: 6px; color: #d1d5db; font-size: 0.75rem; cursor: pointer; transition: background 0.2s; }
  .action-btn:hover { background: #4b5563; }
  .selected-count { font-size: 0.75rem; color: #9ca3af; }

  .thumbnails-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); gap: 0.75rem; }
  .thumbnail-card { position: relative; display: flex; flex-direction: column; background: #111827; border: 2px solid transparent; border-radius: 8px; padding: 0.75rem; cursor: pointer; transition: all 0.2s; text-align: left; }
  .thumbnail-card:hover { background: #1f2937; border-color: #4b5563; }
  .thumbnail-card.active { border-color: #3b82f6; background: #1e3a5f; }
  .thumbnail-card.selected { background: #1e3a5f; }
  .thumbnail-card.selected:not(.active) { border-color: #22c55e; }

  .thumbnail-preview { display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.5rem; }
  .thumbnail-emoji { font-size: 1.5rem; }
  .thumbnail-time { font-size: 0.75rem; color: #9ca3af; font-family: monospace; }
  .thumbnail-info { display: flex; justify-content: space-between; align-items: center; }
  .thumbnail-id { font-size: 0.75rem; color: #d1d5db; font-weight: 500; }
  .thumbnail-score { font-size: 0.875rem; font-weight: 700; }
  .thumbnail-score.score-high { color: #22c55e; }
  .thumbnail-score.score-medium { color: #eab308; }
  .thumbnail-score.score-low { color: #ef4444; }

  .thumbnail-checkbox { position: absolute; top: 0.5rem; right: 0.5rem; }
  .thumbnail-checkbox input { width: 16px; height: 16px; cursor: pointer; }

  .now-playing { position: absolute; bottom: 0; left: 0; right: 0; background: #3b82f6; color: white; font-size: 0.625rem; text-align: center; padding: 0.25rem; border-radius: 0 0 6px 6px; font-weight: 600; }
</style>
