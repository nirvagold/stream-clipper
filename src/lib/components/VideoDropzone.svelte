<script lang="ts">
  import { projectStore } from '$lib/stores';
  import { getVideoInfo, pickFile } from '$lib/utils/tauri';

  let isDragging = $state(false);
  let isLoading = $state(false);
  let error = $state<string | null>(null);

  const SUPPORTED_FORMATS = ['mp4', 'mkv', 'mov', 'webm', 'avi'];

  async function handleFile(path: string) {
    const ext = path.split('.').pop()?.toLowerCase();
    if (!ext || !SUPPORTED_FORMATS.includes(ext)) {
      error = `Unsupported format. Please use: ${SUPPORTED_FORMATS.join(', ')}`;
      return;
    }

    isLoading = true;
    error = null;

    try {
      const info = await getVideoInfo(path);
      projectStore.setVideo(path, info);
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    } finally {
      isLoading = false;
    }
  }

  async function handleBrowse() {
    try {
      const path = await pickFile([
        { name: 'Video Files', extensions: SUPPORTED_FORMATS }
      ]);
      if (path) {
        await handleFile(path);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    const file = e.dataTransfer?.files[0];
    if (file) {
      // In Tauri, we need the file path
      // For now, show message to use browse
      error = 'Please use the Browse button to select a file';
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave() {
    isDragging = false;
  }
</script>

<div 
  class="dropzone"
  class:dragging={isDragging}
  class:loading={isLoading}
  ondrop={handleDrop}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  role="button"
  tabindex="0"
  onclick={handleBrowse}
  onkeydown={(e) => e.key === 'Enter' && handleBrowse()}
>
  {#if isLoading}
    <div class="loading-spinner"></div>
    <p>Loading video info...</p>
  {:else}
    <div class="dropzone-icon">📁</div>
    <p class="dropzone-title">Drop video file here</p>
    <p class="dropzone-subtitle">or click to browse</p>
    <p class="dropzone-formats">Supported: {SUPPORTED_FORMATS.join(', ').toUpperCase()}</p>
  {/if}
  
  {#if error}
    <p class="error">{error}</p>
  {/if}
</div>

<style>
  .dropzone {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem 2rem;
    background: #1f2937;
    border: 2px dashed #4b5563;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .dropzone:hover,
  .dropzone.dragging {
    border-color: #3b82f6;
    background: #1e3a5f;
  }

  .dropzone.loading {
    cursor: wait;
  }

  .dropzone-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .dropzone-title {
    font-size: 1.125rem;
    font-weight: 500;
    color: #f3f4f6;
    margin: 0;
  }

  .dropzone-subtitle {
    font-size: 0.875rem;
    color: #9ca3af;
    margin: 0.25rem 0 0;
  }

  .dropzone-formats {
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 1rem;
  }

  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 3px solid #374151;
    border-top-color: #3b82f6;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .error {
    color: #ef4444;
    font-size: 0.875rem;
    margin-top: 1rem;
  }
</style>
