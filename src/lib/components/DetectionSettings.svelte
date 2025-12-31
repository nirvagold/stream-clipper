<script lang="ts">
  import { settingsStore, licenseStore } from '$lib/stores';
  import { Slider, ProBadge } from './shared';
  import { KeywordEditorModal } from './modals';

  const isPro = $derived($licenseStore.isPro);
  const settings = $derived($settingsStore);
  
  let showKeywordModal = $state(false);

  function handleAudioSensitivityChange(value: number) {
    settingsStore.setAudioSensitivity(value);
  }

  function handleChatRateChange(value: number) {
    settingsStore.setChatRateMultiplier(value);
  }

  function openKeywordEditor() {
    if (isPro) showKeywordModal = true;
  }
</script>

<div class="detection-settings">
  <h3 class="section-title">⚙️ Detection Settings</h3>
  
  <div class="settings-grid">
    <div class="setting-item">
      <Slider
        label="Audio Sensitivity"
        value={settings.audioSensitivity}
        min={1.0}
        max={4.0}
        step={0.1}
        onchange={handleAudioSensitivityChange}
      />
      <p class="setting-hint">Lower = more highlights, Higher = fewer highlights</p>
    </div>

    <div class="setting-item">
      <div class="setting-header">
        <span>Chat Rate Multiplier</span>
        <ProBadge feature="Chat Detection" />
      </div>
      <Slider
        value={settings.chatRateMultiplier}
        min={1.0}
        max={5.0}
        step={0.1}
        disabled={!isPro}
        onchange={handleChatRateChange}
      />
      <p class="setting-hint">Threshold for chat activity spikes</p>
    </div>

    <div class="setting-item keywords-setting">
      <div class="setting-header">
        <span>Keywords</span>
        <ProBadge feature="Custom Keywords" />
      </div>
      <div class="keywords-preview">
        {#each settings.chatKeywords.slice(0, 5) as keyword}
          <span class="keyword-tag">{keyword}</span>
        {/each}
        {#if settings.chatKeywords.length > 5}
          <span class="keyword-more">+{settings.chatKeywords.length - 5} more</span>
        {/if}
      </div>
      <button class="edit-keywords-btn" disabled={!isPro} onclick={openKeywordEditor}>
        Edit Keywords
      </button>
    </div>
  </div>
</div>

{#if showKeywordModal}
  <KeywordEditorModal onClose={() => showKeywordModal = false} />
{/if}

<style>
  .detection-settings {
    background: #1f2937;
    border-radius: 8px;
    padding: 1.25rem;
  }

  .section-title {
    font-size: 1rem;
    font-weight: 600;
    color: #f3f4f6;
    margin: 0 0 1rem;
  }

  .settings-grid {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .setting-item {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .setting-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: #d1d5db;
  }

  .setting-hint {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0;
  }

  .keywords-preview {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
  }

  .keyword-tag {
    padding: 0.25rem 0.5rem;
    background: #374151;
    border-radius: 4px;
    font-size: 0.75rem;
    color: #d1d5db;
  }

  .keyword-more {
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    color: #9ca3af;
  }

  .edit-keywords-btn {
    align-self: flex-start;
    padding: 0.375rem 0.75rem;
    background: #374151;
    border: none;
    border-radius: 6px;
    color: #d1d5db;
    font-size: 0.75rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .edit-keywords-btn:hover:not(:disabled) {
    background: #4b5563;
  }

  .edit-keywords-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
