<script lang="ts">
  interface Props {
    value: number; // 0-100
    showLabel?: boolean;
    size?: 'sm' | 'md' | 'lg';
  }

  let { value = 0, showLabel = true, size = 'md' }: Props = $props();
  
  const clampedValue = $derived(Math.min(100, Math.max(0, value)));
</script>

<div class="progress-container progress-{size}">
  <div class="progress-bar">
    <div class="progress-fill" style="width: {clampedValue}%"></div>
  </div>
  {#if showLabel}
    <span class="progress-label">{Math.round(clampedValue)}%</span>
  {/if}
</div>

<style>
  .progress-container {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
  }

  .progress-bar {
    flex: 1;
    background: #374151;
    border-radius: 9999px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6, #8b5cf6);
    border-radius: 9999px;
    transition: width 0.3s ease;
  }

  .progress-label {
    font-size: 0.875rem;
    color: #9ca3af;
    min-width: 3rem;
    text-align: right;
  }

  /* Sizes */
  .progress-sm .progress-bar { height: 4px; }
  .progress-md .progress-bar { height: 8px; }
  .progress-lg .progress-bar { height: 12px; }
</style>
