<script lang="ts">
  import { highlightsStore, projectStore } from '$lib/stores';
  import { formatDuration } from '$lib/utils/format';

  const waveform = $derived($highlightsStore.waveformData);
  const highlights = $derived($highlightsStore.items);
  const videoInfo = $derived($projectStore.videoInfo);
  const duration = $derived(videoInfo?.duration_secs ?? 0);

  let canvasEl = $state<HTMLCanvasElement | null>(null);
  let containerWidth = $state(0);

  // Draw waveform when data changes
  $effect(() => {
    if (canvasEl && waveform.length > 0 && containerWidth > 0) {
      drawWaveform();
    }
  });

  function drawWaveform() {
    if (!canvasEl) return;
    
    const ctx = canvasEl.getContext('2d');
    if (!ctx) return;

    const width = containerWidth;
    const height = 80;
    canvasEl.width = width;
    canvasEl.height = height;

    // Clear canvas
    ctx.fillStyle = '#1f2937';
    ctx.fillRect(0, 0, width, height);

    // Draw waveform bars
    const barWidth = Math.max(1, width / waveform.length);
    const maxVal = Math.max(...waveform, 0.01);

    ctx.fillStyle = '#4b5563';
    waveform.forEach((val, i) => {
      const x = (i / waveform.length) * width;
      const barHeight = (val / maxVal) * (height - 10);
      const y = (height - barHeight) / 2;
      ctx.fillRect(x, y, barWidth - 1, barHeight);
    });

    // Draw highlight regions
    highlights.forEach(h => {
      const startX = (h.start_secs / duration) * width;
      const endX = (h.end_secs / duration) * width;
      
      // Color based on type
      let color = 'rgba(59, 130, 246, 0.4)'; // blue for audio
      if (h.highlight_type === 'Chat') {
        color = 'rgba(16, 185, 129, 0.4)'; // green for chat
      } else if (h.highlight_type === 'Combo') {
        color = 'rgba(245, 158, 11, 0.4)'; // amber for combo
      }
      
      ctx.fillStyle = color;
      ctx.fillRect(startX, 0, endX - startX, height);
    });
  }

  function handleResize(el: HTMLDivElement) {
    const observer = new ResizeObserver(entries => {
      containerWidth = entries[0].contentRect.width;
    });
    observer.observe(el);
    return {
      destroy() {
        observer.disconnect();
      }
    };
  }
</script>

<div class="timeline" use:handleResize>
  <div class="timeline-header">
    <span class="timeline-title">📊 Timeline</span>
    {#if duration > 0}
      <span class="timeline-duration">{formatDuration(duration)}</span>
    {/if}
  </div>
  
  <div class="timeline-canvas-container">
    <canvas bind:this={canvasEl} class="timeline-canvas"></canvas>
    
    <!-- Highlight markers -->
    <div class="markers">
      {#each highlights as h, i}
        {@const left = (h.start_secs / duration) * 100}
        <div 
          class="marker"
          class:audio={h.highlight_type === 'Audio'}
          class:chat={h.highlight_type === 'Chat'}
          class:combo={h.highlight_type === 'Combo'}
          style="left: {left}%"
          title="Clip #{i + 1} - {formatDuration(h.start_secs)}"
        >
          {i + 1}
        </div>
      {/each}
    </div>
  </div>

  <!-- Legend -->
  <div class="legend">
    <span class="legend-item audio">🔊 Audio</span>
    <span class="legend-item chat">💬 Chat</span>
    <span class="legend-item combo">⚡ Combo</span>
  </div>
</div>

<style>
  .timeline {
    background: #1f2937;
    border-radius: 8px;
    padding: 1rem;
  }

  .timeline-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .timeline-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: #f3f4f6;
  }

  .timeline-duration {
    font-size: 0.75rem;
    color: #9ca3af;
    font-family: monospace;
  }

  .timeline-canvas-container {
    position: relative;
    width: 100%;
    height: 80px;
    border-radius: 4px;
    overflow: hidden;
  }

  .timeline-canvas {
    width: 100%;
    height: 100%;
    display: block;
  }

  .markers {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    pointer-events: none;
  }

  .marker {
    position: absolute;
    top: -8px;
    transform: translateX(-50%);
    width: 18px;
    height: 18px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.625rem;
    font-weight: 600;
    color: white;
    pointer-events: auto;
    cursor: pointer;
  }

  .marker.audio { background: #3b82f6; }
  .marker.chat { background: #10b981; }
  .marker.combo { background: #f59e0b; }

  .legend {
    display: flex;
    gap: 1rem;
    margin-top: 0.75rem;
    justify-content: center;
  }

  .legend-item {
    font-size: 0.75rem;
    color: #9ca3af;
  }
</style>
