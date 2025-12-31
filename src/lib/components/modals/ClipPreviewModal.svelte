<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { Highlight } from '$lib/types';
  import { previewClip } from '$lib/utils/tauri';
  import { formatDuration, getHighlightEmoji } from '$lib/utils/format';
  import { Button } from '../shared';

  interface Props {
    highlight: Highlight;
    videoPath: string;
    onClose: () => void;
  }

  let { highlight, videoPath, onClose }: Props = $props();

  let previewPath = $state<string | null>(null);
  let isLoading = $state(true);
  let error = $state<string | null>(null);
  let videoElement = $state<HTMLVideoElement | null>(null);

  onMount(async () => {
    try {
      // Generate preview clip
      previewPath = await previewClip(
        videoPath,
        highlight.start_secs,
        highlight.end_secs
      );
      isLoading = false;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      isLoading = false;
    }
  });

  onDestroy(() => {
    // Cleanup: pause video if playing
    if (videoElement) {
      videoElement.pause();
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onClose();
    }
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }

  function handleBackdropKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div 
  class="modal-backdrop" 
  onclick={handleBackdropClick}
  onkeydown={handleBackdropKeydown}
  role="dialog"
  tabindex="-1"
  aria-modal="true"
  aria-labelledby="preview-title"
>
  <div class="modal">
    <div class="modal-header">
      <h2 id="preview-title">
        {getHighlightEmoji(highlight.highlight_type)} Clip #{highlight.id} Preview
      </h2>
      <button class="close-btn" onclick={onClose} aria-label="Close">×</button>
    </div>

    <div class="modal-content">
      {#if isLoading}
        <div class="loading">
          <div class="spinner"></div>
          <p>Generating preview...</p>
        </div>
      {:else if error}
        <div class="error">
          <p>❌ Failed to generate preview</p>
          <p class="error-detail">{error}</p>
        </div>
      {:else if previewPath}
        <div class="video-container">
          <video 
            bind:this={videoElement}
            src={`asset://localhost/${previewPath}`}
            controls
            autoplay
            controlslist="nodownload noplaybackrate"
            disablepictureinpicture
            oncontextmenu={(e) => e.preventDefault()}
            class="preview-video"
          >
            <track kind="captions" />
          </video>
        </div>
      {/if}

      <div class="clip-info">
        <div class="info-row">
          <span class="label">Start:</span>
          <span class="value">{formatDuration(highlight.start_secs)}</span>
        </div>
        <div class="info-row">
          <span class="label">End:</span>
          <span class="value">{formatDuration(highlight.end_secs)}</span>
        </div>
        <div class="info-row">
          <span class="label">Duration:</span>
          <span class="value">{formatDuration(highlight.duration_secs)}</span>
        </div>
        <div class="info-row">
          <span class="label">Score:</span>
          <span class="value score">{Math.round(highlight.score)}</span>
        </div>
        <div class="info-row">
          <span class="label">Type:</span>
          <span class="value">{highlight.highlight_type}</span>
        </div>
      </div>
    </div>

    <div class="modal-footer">
      <Button variant="secondary" onclick={onClose}>Close</Button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 1rem;
  }

  .modal {
    background: #1f2937;
    border-radius: 12px;
    width: 100%;
    max-width: 800px;
    max-height: 90vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid #374151;
  }

  .modal-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #f3f4f6;
    margin: 0;
  }

  .close-btn {
    background: none;
    border: none;
    color: #9ca3af;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0;
    line-height: 1;
  }

  .close-btn:hover {
    color: #f3f4f6;
  }

  .modal-content {
    padding: 1.5rem;
    overflow-y: auto;
    flex: 1;
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 3rem;
    color: #9ca3af;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid #374151;
    border-top-color: #3b82f6;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .error {
    text-align: center;
    padding: 2rem;
    color: #ef4444;
  }

  .error-detail {
    font-size: 0.875rem;
    color: #9ca3af;
    margin-top: 0.5rem;
  }

  .video-container {
    background: #000;
    border-radius: 8px;
    overflow: hidden;
    margin-bottom: 1.5rem;
  }

  .preview-video {
    width: 100%;
    max-height: 400px;
    display: block;
  }

  .clip-info {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 0.75rem;
    background: #111827;
    border-radius: 8px;
    padding: 1rem;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .label {
    font-size: 0.875rem;
    color: #9ca3af;
  }

  .value {
    font-size: 0.875rem;
    color: #f3f4f6;
    font-weight: 500;
    font-family: monospace;
  }

  .value.score {
    color: #22c55e;
    font-weight: 700;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    padding: 1rem 1.5rem;
    border-top: 1px solid #374151;
  }
</style>
