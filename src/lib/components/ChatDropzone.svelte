<script lang="ts">
  import { projectStore, licenseStore } from '$lib/stores';
  import { getChatInfo, pickFile } from '$lib/utils/tauri';
  import { ProBadge } from './shared';

  let isLoading = $state(false);
  let error = $state<string | null>(null);

  const isPro = $derived($licenseStore.isPro);
  const SUPPORTED_FORMATS = ['json', 'txt'];

  async function handleFile(path: string) {
    if (!isPro) {
      error = 'Chat detection requires Pro license';
      return;
    }

    isLoading = true;
    error = null;

    try {
      const info = await getChatInfo(path);
      projectStore.setChat(path, info);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      isLoading = false;
    }
  }

  async function handleBrowse() {
    if (!isPro) {
      error = 'Chat detection requires Pro license';
      return;
    }

    try {
      const path = await pickFile([
        { name: 'Chat Files', extensions: SUPPORTED_FORMATS }
      ]);
      if (path) {
        await handleFile(path);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }
</script>

<div 
  class="chat-dropzone"
  class:disabled={!isPro}
  class:loading={isLoading}
  role="button"
  tabindex="0"
  onclick={handleBrowse}
  onkeydown={(e) => e.key === 'Enter' && handleBrowse()}
>
  <div class="chat-header">
    <span class="chat-icon">📄</span>
    <span class="chat-title">Chat Log (Optional)</span>
    <ProBadge feature="Chat Detection" />
  </div>
  
  {#if isLoading}
    <p class="chat-subtitle">Loading chat info...</p>
  {:else}
    <p class="chat-subtitle">
      {#if isPro}
        Click to import Twitch/YouTube chat
      {:else}
        Upgrade to Pro to enable chat detection
      {/if}
    </p>
  {/if}
  
  {#if error}
    <p class="error">{error}</p>
  {/if}
</div>

<style>
  .chat-dropzone {
    display: flex;
    flex-direction: column;
    padding: 1rem 1.5rem;
    background: #1f2937;
    border: 1px dashed #4b5563;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .chat-dropzone:hover:not(.disabled) {
    border-color: #3b82f6;
    background: #1e3a5f;
  }

  .chat-dropzone.disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .chat-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .chat-icon {
    font-size: 1.25rem;
  }

  .chat-title {
    font-weight: 500;
    color: #f3f4f6;
  }

  .chat-subtitle {
    font-size: 0.75rem;
    color: #9ca3af;
    margin: 0.25rem 0 0;
  }

  .error {
    color: #ef4444;
    font-size: 0.75rem;
    margin-top: 0.5rem;
  }
</style>
