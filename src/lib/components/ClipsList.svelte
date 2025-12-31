<script lang="ts">
  import { highlightsStore, selectedCount } from '$lib/stores';
  import ClipCard from './ClipCard.svelte';

  const highlights = $derived($highlightsStore.items);
  const count = $derived($selectedCount);
  const total = $derived(highlights.length);

  function selectAll() {
    highlightsStore.selectAll();
  }

  function deselectAll() {
    highlightsStore.deselectAll();
  }
</script>

<div class="clips-list">
  <div class="clips-header">
    <h3 class="section-title">🎬 Detected Clips ({total} found)</h3>
    <div class="clips-actions">
      <button class="action-btn" onclick={selectAll}>Select All</button>
      <button class="action-btn" onclick={deselectAll}>Deselect All</button>
      <span class="selected-count">{count} selected</span>
    </div>
  </div>

  {#if highlights.length === 0}
    <div class="empty-state">
      <p>No highlights detected yet.</p>
      <p class="hint">Click "Analyze Video" to find highlights.</p>
    </div>
  {:else}
    <div class="clips-grid">
      {#each highlights as highlight (highlight.id)}
        <ClipCard {highlight} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .clips-list {
    background: #1f2937;
    border-radius: 8px;
    padding: 1.25rem;
  }

  .clips-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1rem;
    flex-wrap: wrap;
    gap: 0.75rem;
  }

  .section-title {
    font-size: 1rem;
    font-weight: 600;
    color: #f3f4f6;
    margin: 0;
  }

  .clips-actions {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .action-btn {
    padding: 0.375rem 0.75rem;
    background: #374151;
    border: none;
    border-radius: 6px;
    color: #d1d5db;
    font-size: 0.75rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .action-btn:hover {
    background: #4b5563;
  }

  .selected-count {
    font-size: 0.75rem;
    color: #9ca3af;
  }

  .clips-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
  }

  .empty-state {
    text-align: center;
    padding: 2rem;
    color: #9ca3af;
  }

  .empty-state p {
    margin: 0;
  }

  .empty-state .hint {
    font-size: 0.875rem;
    color: #6b7280;
    margin-top: 0.5rem;
  }
</style>
