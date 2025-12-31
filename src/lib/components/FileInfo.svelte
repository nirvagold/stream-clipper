<script lang="ts">
  import { projectStore } from '$lib/stores';
  import { formatDuration, formatFileSize } from '$lib/utils/format';

  const videoInfo = $derived($projectStore.videoInfo);
  const chatInfo = $derived($projectStore.chatInfo);

  function removeVideo() {
    projectStore.removeVideo();
  }

  function removeChat() {
    projectStore.removeChat();
  }
</script>

<div class="file-info">
  {#if videoInfo}
    <div class="file-card">
      <div class="file-icon">📹</div>
      <div class="file-details">
        <p class="file-name">{videoInfo.filename}</p>
        <p class="file-meta">
          {formatDuration(videoInfo.duration_secs)} • 
          {videoInfo.width}x{videoInfo.height} • 
          {videoInfo.fps.toFixed(0)}fps • 
          {formatFileSize(videoInfo.file_size_bytes)}
        </p>
      </div>
      <button class="remove-btn" onclick={removeVideo} title="Remove video">✕</button>
    </div>
  {/if}

  {#if chatInfo}
    <div class="file-card">
      <div class="file-icon">📄</div>
      <div class="file-details">
        <p class="file-name">{chatInfo.path.split(/[/\\]/).pop()}</p>
        <p class="file-meta">
          {chatInfo.total_messages.toLocaleString()} messages • 
          {formatDuration(chatInfo.duration_secs)} • 
          {chatInfo.avg_rate_per_min.toFixed(0)} msg/min
        </p>
      </div>
      <button class="remove-btn" onclick={removeChat} title="Remove chat">✕</button>
    </div>
  {/if}
</div>

<style>
  .file-info {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .file-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: #1f2937;
    border-radius: 8px;
  }

  .file-icon {
    font-size: 1.5rem;
  }

  .file-details {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    font-weight: 500;
    color: #f3f4f6;
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-meta {
    font-size: 0.75rem;
    color: #9ca3af;
    margin: 0.25rem 0 0;
  }

  .remove-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: #9ca3af;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .remove-btn:hover {
    background: #374151;
    color: #ef4444;
  }
</style>
