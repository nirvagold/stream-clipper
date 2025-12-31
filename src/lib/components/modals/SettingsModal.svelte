<script lang="ts">
  import { settingsStore } from '$lib/stores';
  import { saveSettings, resetSettings, pickFolder } from '$lib/utils/tauri';
  import { Button, Slider } from '../shared';

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  const settings = $derived($settingsStore);
  
  let isLoading = $state(false);
  let error = $state<string | null>(null);
  let success = $state<string | null>(null);

  async function handleSave() {
    isLoading = true;
    error = null;

    try {
      const apiSettings = {
        version: 1,
        detection: settingsStore.toAnalyzeSettings(settings),
        export: settingsStore.toExportSettings(settings, false),
      };
      await saveSettings(apiSettings);
      success = 'Settings saved!';
      setTimeout(() => success = null, 2000);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save settings';
    } finally {
      isLoading = false;
    }
  }

  async function handleReset() {
    isLoading = true;
    error = null;

    try {
      const defaultSettings = await resetSettings();
      settingsStore.loadSettings(defaultSettings);
      success = 'Settings reset to defaults';
      setTimeout(() => success = null, 2000);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to reset settings';
    } finally {
      isLoading = false;
    }
  }

  async function handlePickFolder() {
    try {
      const folder = await pickFolder();
      if (folder) {
        settingsStore.setOutputFolder(folder);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to pick folder';
    }
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onClose();
    }
  }
</script>

<div 
  class="modal-backdrop" 
  onclick={handleBackdropClick} 
  onkeydown={handleKeydown}
  role="dialog" 
  aria-modal="true"
  tabindex="-1"
>
  <div class="modal">
    <div class="modal-header">
      <h2>⚙️ Settings</h2>
      <button class="close-btn" onclick={onClose}>✕</button>
    </div>

    <div class="modal-body">
      <!-- Detection Settings -->
      <section class="settings-section">
        <h3>Detection</h3>
        
        <Slider
          label="Audio Sensitivity"
          value={settings.audioSensitivity}
          min={1.0}
          max={4.0}
          step={0.1}
          onchange={(v) => settingsStore.setAudioSensitivity(v)}
        />

        <Slider
          label="Min Clip Duration (sec)"
          value={settings.audioMinDuration}
          min={1.0}
          max={10.0}
          step={0.5}
          onchange={(v) => settingsStore.setAudioMinDuration(v)}
        />

        <Slider
          label="Merge Gap (sec)"
          value={settings.audioMergeGap}
          min={1.0}
          max={10.0}
          step={0.5}
          onchange={(v) => settingsStore.setAudioMergeGap(v)}
        />

        <Slider
          label="Chat Rate Multiplier"
          value={settings.chatRateMultiplier}
          min={1.0}
          max={5.0}
          step={0.5}
          onchange={(v) => settingsStore.setChatRateMultiplier(v)}
        />
      </section>

      <!-- Scoring Weights -->
      <section class="settings-section">
        <h3>Scoring Weights</h3>
        
        <Slider
          label="Audio Weight"
          value={settings.audioWeight}
          min={0}
          max={1}
          step={0.1}
          onchange={(v) => settingsStore.setAudioWeight(v)}
        />

        <Slider
          label="Chat Weight"
          value={settings.chatWeight}
          min={0}
          max={1}
          step={0.1}
          onchange={(v) => settingsStore.setChatWeight(v)}
        />

        <Slider
          label="Combo Bonus"
          value={settings.comboBonus}
          min={1.0}
          max={3.0}
          step={0.1}
          onchange={(v) => settingsStore.setComboBonus(v)}
        />
      </section>

      <!-- Export Settings -->
      <section class="settings-section">
        <h3>Export</h3>
        
        <Slider
          label="Padding Before (sec)"
          value={settings.paddingBefore}
          min={0}
          max={10}
          step={0.5}
          onchange={(v) => settingsStore.setPaddingBefore(v)}
        />

        <Slider
          label="Padding After (sec)"
          value={settings.paddingAfter}
          min={0}
          max={10}
          step={0.5}
          onchange={(v) => settingsStore.setPaddingAfter(v)}
        />

        <div class="folder-setting">
          <label for="default-folder">Default Output Folder:</label>
          <div class="folder-row">
            <input
              id="default-folder"
              type="text"
              value={settings.outputFolder}
              readonly
              placeholder="Not set"
            />
            <button class="browse-btn" onclick={handlePickFolder}>Browse</button>
          </div>
        </div>
      </section>

      {#if error}
        <p class="error">{error}</p>
      {/if}
      {#if success}
        <p class="success">{success}</p>
      {/if}
    </div>

    <div class="modal-footer">
      <Button variant="ghost" onclick={handleReset} disabled={isLoading}>
        Reset to Defaults
      </Button>
      <div class="footer-right">
        <Button variant="secondary" onclick={onClose}>
          Cancel
        </Button>
        <Button variant="primary" onclick={handleSave} loading={isLoading}>
          Save
        </Button>
      </div>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal {
    background: #1f2937;
    border-radius: 12px;
    width: 90%;
    max-width: 500px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.25rem;
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
    font-size: 1.25rem;
    cursor: pointer;
    padding: 0.25rem;
  }

  .close-btn:hover {
    color: #f3f4f6;
  }

  .modal-body {
    padding: 1.25rem;
    overflow-y: auto;
    flex: 1;
  }

  .settings-section {
    margin-bottom: 1.5rem;
  }

  .settings-section:last-child {
    margin-bottom: 0;
  }

  .settings-section h3 {
    font-size: 0.875rem;
    font-weight: 600;
    color: #d1d5db;
    margin: 0 0 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid #374151;
  }

  .folder-setting {
    margin-top: 1rem;
  }

  .folder-setting label {
    display: block;
    font-size: 0.875rem;
    color: #d1d5db;
    margin-bottom: 0.5rem;
  }

  .folder-row {
    display: flex;
    gap: 0.5rem;
  }

  .folder-row input {
    flex: 1;
    padding: 0.5rem 0.75rem;
    background: #374151;
    border: 1px solid #4b5563;
    border-radius: 6px;
    color: #f3f4f6;
    font-size: 0.875rem;
  }

  .browse-btn {
    padding: 0.5rem 0.75rem;
    background: #374151;
    border: none;
    border-radius: 6px;
    color: #d1d5db;
    font-size: 0.875rem;
    cursor: pointer;
  }

  .browse-btn:hover {
    background: #4b5563;
  }

  .modal-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    border-top: 1px solid #374151;
  }

  .footer-right {
    display: flex;
    gap: 0.75rem;
  }

  .error {
    color: #ef4444;
    font-size: 0.875rem;
    margin-top: 1rem;
  }

  .success {
    color: #10b981;
    font-size: 0.875rem;
    margin-top: 1rem;
  }
</style>
