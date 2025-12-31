<script lang="ts">
  import { settingsStore, licenseStore, highlightsStore, selectedHighlights, selectedCount, projectStore, uiStore } from '$lib/stores';
  import { exportClips, pickFolder, openFolder, onExportProgress } from '$lib/utils/tauri';
  import { Button, ProgressBar, ProBadge } from './shared';
  import type { ClipExport } from '$lib/types';

  const isPro = $derived($licenseStore.isPro);
  const settings = $derived($settingsStore);
  const selected = $derived($selectedHighlights);
  const count = $derived($selectedCount);
  const isExporting = $derived($highlightsStore.isExporting);
  const exportProgress = $derived($highlightsStore.exportProgress);
  const videoPath = $derived($projectStore.videoPath);

  let error = $state<string | null>(null);

  async function handlePickFolder() {
    try {
      const folder = await pickFolder();
      if (folder) {
        settingsStore.setOutputFolder(folder);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      uiStore.error('Failed to select folder');
    }
  }

  async function handleExport() {
    if (!videoPath || selected.length === 0) return;

    error = null;
    highlightsStore.startExport(selected.length);

    // Setup progress listener
    const unlisten = await onExportProgress((event) => {
      highlightsStore.updateExportProgress(event.current, event.percent);
    });

    try {
      const clips: ClipExport[] = selected.map(h => ({
        highlight_id: h.id,
        start_secs: Math.max(0, h.start_secs - settings.paddingBefore),
        end_secs: h.end_secs + settings.paddingAfter,
      }));

      const exportSettings = settingsStore.toExportSettings(settings, !isPro);
      const results = await exportClips(videoPath, clips, exportSettings);

      const failed = results.filter(r => !r.success);
      const succeeded = results.filter(r => r.success);
      
      if (failed.length > 0) {
        error = `${failed.length} clip(s) failed to export`;
        uiStore.warning(`Exported ${succeeded.length} clips, ${failed.length} failed`);
      } else {
        uiStore.success(`Successfully exported ${succeeded.length} clip${succeeded.length > 1 ? 's' : ''}!`);
      }

      highlightsStore.finishExport();
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      highlightsStore.finishExport();
      uiStore.error(`Export failed: ${error}`);
    } finally {
      unlisten();
    }
  }

  async function handleOpenFolder() {
    if (settings.outputFolder) {
      await openFolder(settings.outputFolder);
    }
  }
</script>

<div class="export-panel">
  <h3 class="section-title">📤 Export Settings</h3>

  <div class="export-options">
    <div class="option-row">
      <label for="export-format">Format:</label>
      <select 
        id="export-format" 
        value={settings.outputFormat}
        onchange={(e) => settingsStore.setOutputFormat(e.currentTarget.value as 'Mp4' | 'WebM')}
      >
        <option value="Mp4">MP4</option>
        <option value="WebM" disabled={!isPro}>WebM {!isPro ? '(Pro)' : ''}</option>
      </select>

      <label for="export-resolution">Resolution:</label>
      <select 
        id="export-resolution" 
        value={settings.outputResolution}
        onchange={(e) => settingsStore.setOutputResolution(e.currentTarget.value as any)}
      >
        <option value="R720p">720p</option>
        <option value="R1080p">1080p</option>
        <option value="R1440p" disabled={!isPro}>1440p {!isPro ? '(Pro)' : ''}</option>
        <option value="R4K" disabled={!isPro}>4K {!isPro ? '(Pro)' : ''}</option>
        <option value="Source">Source</option>
      </select>
    </div>

    <div class="option-row checkboxes">
      <label class="checkbox-label">
        <input 
          type="checkbox" 
          checked={settings.verticalCrop}
          disabled={!isPro}
          onchange={(e) => settingsStore.setVerticalCrop(e.currentTarget.checked)}
        />
        <span>Vertical crop (9:16)</span>
        <ProBadge feature="Vertical Crop" />
      </label>

      <label class="checkbox-label">
        <input 
          type="checkbox" 
          checked={settings.fadeEffect}
          disabled={!isPro}
          onchange={(e) => settingsStore.setFadeEffect(e.currentTarget.checked)}
        />
        <span>Add fade in/out</span>
        <ProBadge feature="Fade Effect" />
      </label>
    </div>

    <div class="folder-row">
      <label for="output-folder">Output:</label>
      <input 
        id="output-folder"
        type="text" 
        value={settings.outputFolder}
        readonly
        placeholder="Select output folder..."
      />
      <button class="browse-btn" onclick={handlePickFolder}>Browse</button>
      {#if settings.outputFolder}
        <button class="open-btn" onclick={handleOpenFolder} title="Open folder">📁</button>
      {/if}
    </div>
  </div>

  {#if isExporting}
    <div class="export-progress">
      <ProgressBar value={exportProgress} />
      <p class="progress-text">Exporting clips...</p>
    </div>
  {/if}

  {#if error}
    <p class="error">{error}</p>
  {/if}

  <div class="export-actions">
    <Button 
      variant="primary" 
      size="lg"
      disabled={count === 0 || !settings.outputFolder || isExporting}
      loading={isExporting}
      onclick={handleExport}
    >
      Export {count} Clip{count !== 1 ? 's' : ''}
    </Button>
  </div>
</div>

<style>
  .export-panel {
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

  .export-options {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .option-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .option-row label {
    font-size: 0.875rem;
    color: #d1d5db;
  }

  .option-row select {
    padding: 0.375rem 0.75rem;
    background: #374151;
    border: 1px solid #4b5563;
    border-radius: 6px;
    color: #f3f4f6;
    font-size: 0.875rem;
  }

  .checkboxes {
    gap: 1.5rem;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }

  .checkbox-label input:disabled {
    cursor: not-allowed;
  }

  .folder-row {
    display: flex;
    align-items: center;
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

  .browse-btn, .open-btn {
    padding: 0.5rem 0.75rem;
    background: #374151;
    border: none;
    border-radius: 6px;
    color: #d1d5db;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .browse-btn:hover, .open-btn:hover {
    background: #4b5563;
  }

  .export-progress {
    margin-top: 1rem;
  }

  .progress-text {
    font-size: 0.75rem;
    color: #9ca3af;
    margin: 0.5rem 0 0;
    text-align: center;
  }

  .error {
    color: #ef4444;
    font-size: 0.875rem;
    margin: 1rem 0 0;
  }

  .export-actions {
    margin-top: 1.5rem;
    display: flex;
    justify-content: center;
  }
</style>
