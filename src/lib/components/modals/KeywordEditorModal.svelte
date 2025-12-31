<script lang="ts">
  import { settingsStore } from '$lib/stores';

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  // Initialize with current keywords from store
  let keywords = $state<string[]>([...$settingsStore.chatKeywords]);
  let newKeyword = $state('');
  let error = $state('');

  function addKeyword() {
    const kw = newKeyword.trim().toUpperCase();
    if (!kw) return;
    
    if (keywords.includes(kw)) {
      error = 'Keyword already exists';
      return;
    }
    
    keywords = [...keywords, kw];
    newKeyword = '';
    error = '';
  }

  function removeKeyword(index: number) {
    keywords = keywords.filter((_, i) => i !== index);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      addKeyword();
    }
  }

  function saveKeywords() {
    settingsStore.setChatKeywords(keywords);
    onClose();
  }

  function resetToDefault() {
    keywords = [
      'POG', 'POGGERS', 'POGU', 'LETS GO', "LET'S GO", 'OMG', 'WTF',
      'CLIP IT', 'CLIP THAT', 'GG', 'GGWP', 'HOLY', 'INSANE', 'CRAZY',
      'NO WAY', 'NOWAY', 'KEKW', 'LULW', 'OMEGALUL', 'HYPE'
    ];
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) onClose();
  }
</script>

<div class="modal-backdrop" onclick={handleBackdropClick} onkeydown={(e) => e.key === 'Escape' && onClose()} role="dialog" aria-modal="true" tabindex="-1">
  <div class="modal">
    <div class="modal-header">
      <h2>🔑 Edit Keywords</h2>
      <button class="close-btn" onclick={onClose}>✕</button>
    </div>

    <div class="modal-body">
      <p class="description">
        Keywords trigger chat highlight detection when viewers type these words.
        Case-insensitive matching.
      </p>

      <div class="add-keyword">
        <input
          type="text"
          placeholder="Add new keyword..."
          bind:value={newKeyword}
          onkeydown={handleKeydown}
        />
        <button class="add-btn" onclick={addKeyword}>Add</button>
      </div>
      
      {#if error}
        <p class="error">{error}</p>
      {/if}

      <div class="keywords-list">
        {#each keywords as keyword, i (keyword)}
          <div class="keyword-item">
            <span>{keyword}</span>
            <button class="remove-btn" onclick={() => removeKeyword(i)}>✕</button>
          </div>
        {/each}
        
        {#if keywords.length === 0}
          <p class="empty">No keywords. Add some above.</p>
        {/if}
      </div>

      <div class="keyword-count">
        {keywords.length} keyword{keywords.length !== 1 ? 's' : ''}
      </div>
    </div>

    <div class="modal-footer">
      <button class="btn btn-secondary" onclick={resetToDefault}>Reset to Default</button>
      <div class="footer-right">
        <button class="btn btn-secondary" onclick={onClose}>Cancel</button>
        <button class="btn btn-primary" onclick={saveKeywords}>Save</button>
      </div>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed; inset: 0; background: rgba(0, 0, 0, 0.7);
    display: flex; align-items: center; justify-content: center; z-index: 100;
  }

  .modal {
    background: #1f2937; border-radius: 12px; width: 90%; max-width: 500px;
    max-height: 80vh; display: flex; flex-direction: column;
  }

  .modal-header {
    display: flex; justify-content: space-between; align-items: center;
    padding: 1.25rem; border-bottom: 1px solid #374151;
  }

  .modal-header h2 { font-size: 1.25rem; font-weight: 600; color: #f3f4f6; margin: 0; }

  .close-btn {
    background: none; border: none; color: #9ca3af; font-size: 1.25rem;
    cursor: pointer; padding: 0.25rem;
  }
  .close-btn:hover { color: #f3f4f6; }

  .modal-body { padding: 1.25rem; overflow-y: auto; flex: 1; }

  .description { font-size: 0.875rem; color: #9ca3af; margin: 0 0 1rem; }

  .add-keyword { display: flex; gap: 0.5rem; margin-bottom: 0.5rem; }

  .add-keyword input {
    flex: 1; padding: 0.625rem 0.875rem; background: #111827;
    border: 1px solid #374151; border-radius: 6px; color: #f3f4f6; font-size: 0.875rem;
  }
  .add-keyword input:focus { outline: none; border-color: #3b82f6; }

  .add-btn {
    padding: 0.625rem 1rem; background: #3b82f6; border: none;
    border-radius: 6px; color: white; font-weight: 500; cursor: pointer;
  }
  .add-btn:hover { background: #2563eb; }

  .error { color: #ef4444; font-size: 0.75rem; margin: 0 0 0.5rem; }

  .keywords-list {
    display: flex; flex-wrap: wrap; gap: 0.5rem;
    max-height: 250px; overflow-y: auto; padding: 0.5rem 0;
  }

  .keyword-item {
    display: flex; align-items: center; gap: 0.375rem;
    padding: 0.375rem 0.625rem; background: #374151; border-radius: 6px;
  }

  .keyword-item span { font-size: 0.8rem; color: #d1d5db; }

  .remove-btn {
    background: none; border: none; color: #9ca3af; font-size: 0.75rem;
    cursor: pointer; padding: 0; line-height: 1;
  }
  .remove-btn:hover { color: #ef4444; }

  .empty { color: #6b7280; font-size: 0.875rem; text-align: center; padding: 1rem; }

  .keyword-count { font-size: 0.75rem; color: #6b7280; margin-top: 0.5rem; }

  .modal-footer {
    display: flex; justify-content: space-between; align-items: center;
    padding: 1rem 1.25rem; border-top: 1px solid #374151;
  }

  .footer-right { display: flex; gap: 0.75rem; }

  .btn {
    padding: 0.625rem 1rem; border: none; border-radius: 6px;
    font-size: 0.875rem; font-weight: 500; cursor: pointer;
  }
  .btn-primary { background: #3b82f6; color: white; }
  .btn-primary:hover { background: #2563eb; }
  .btn-secondary { background: #374151; color: #d1d5db; }
  .btn-secondary:hover { background: #4b5563; }
</style>
