<script lang="ts">
  import { projectStore, settingsStore, highlightsStore, licenseStore, uiStore } from '$lib/stores';
  import { analyzeVideo, onAnalyzeProgress, getDefaultOutputFolder } from '$lib/utils/tauri';
  import { Button, ProgressBar } from './shared';
  import FileInfo from './FileInfo.svelte';
  import DetectionSettings from './DetectionSettings.svelte';
  import Timeline from './Timeline.svelte';
  import ClipsTheater from './ClipsTheater.svelte';
  import ExportPanel from './ExportPanel.svelte';
  import ChatDropzone from './ChatDropzone.svelte';
  import { onMount } from 'svelte';

  const project = $derived($projectStore);
  const settings = $derived($settingsStore);
  const highlights = $derived($highlightsStore.items);
  const isPro = $derived($licenseStore.isPro);

  let error = $state<string | null>(null);

  onMount(async () => {
    // Set default output folder if not set
    if (!settings.outputFolder) {
      const folder = await getDefaultOutputFolder();
      settingsStore.setOutputFolder(folder);
    }
  });

  async function handleAnalyze() {
    if (!project.videoPath) return;

    error = null;
    projectStore.startAnalysis();

    // Setup progress listener
    const unlisten = await onAnalyzeProgress((event) => {
      projectStore.updateProgress(event.stage as any, event.progress);
    });

    try {
      const analyzeSettings = settingsStore.toAnalyzeSettings(settings);
      
      // Apply free tier limit
      if (!isPro) {
        analyzeSettings.max_clips = 5;
      }

      const result = await analyzeVideo(
        project.videoPath,
        isPro ? project.chatPath : null,
        analyzeSettings
      );

      highlightsStore.setHighlights(result.highlights, result.waveform_data);
      projectStore.finishAnalysis();
      
      // Show success toast
      if (result.highlights.length > 0) {
        uiStore.success(`Found ${result.highlights.length} highlight${result.highlights.length > 1 ? 's' : ''}!`);
      } else {
        uiStore.info('No highlights detected. Try lowering the sensitivity.');
      }
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      projectStore.setError(error);
      uiStore.error(`Analysis failed: ${error}`);
    } finally {
      unlisten();
    }
  }

  function handleNewVideo() {
    projectStore.reset();
    highlightsStore.reset();
  }
</script>

<div class="workspace">
  <div class="workspace-main">
    <!-- File Info Section -->
    <section class="section">
      <FileInfo />
      {#if !project.chatInfo}
        <div class="chat-import">
          <ChatDropzone />
        </div>
      {/if}
    </section>

    <!-- Detection Settings -->
    <section class="section">
      <DetectionSettings />
    </section>

    <!-- Analyze Button -->
    <section class="section analyze-section">
      {#if project.isAnalyzing}
        <div class="analyze-progress">
          <ProgressBar value={project.analyzeProgress} />
          <p class="progress-stage">
            {#if project.analyzeStage === 'extracting'}
              Extracting audio...
            {:else if project.analyzeStage === 'analyzing_audio'}
              Analyzing audio for highlights...
            {:else if project.analyzeStage === 'analyzing_chat'}
              Analyzing chat activity...
            {:else if project.analyzeStage === 'scoring'}
              Scoring and ranking highlights...
            {:else}
              Processing...
            {/if}
          </p>
        </div>
      {:else}
        <Button 
          variant="primary" 
          size="lg"
          onclick={handleAnalyze}
          disabled={!project.videoPath}
        >
          🔍 Analyze Video
        </Button>
      {/if}

      {#if error}
        <p class="error">{error}</p>
      {/if}
    </section>

    <!-- Clips List -->
    {#if highlights.length > 0}
      <!-- Timeline -->
      <section class="section">
        <Timeline />
      </section>

      <!-- Theater View with Clips -->
      <section class="section">
        <ClipsTheater />
      </section>

      <!-- Export Panel -->
      <section class="section">
        <ExportPanel />
      </section>
    {/if}
  </div>

  <!-- Footer Actions -->
  <div class="workspace-footer">
    <Button variant="secondary" onclick={handleNewVideo}>
      📁 New Video
    </Button>
  </div>
</div>

<style>
  .workspace {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 1.5rem;
    overflow-y: auto;
  }

  .workspace-main {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 900px;
    margin: 0 auto;
    width: 100%;
  }

  .section {
    width: 100%;
  }

  .chat-import {
    margin-top: 0.75rem;
  }

  .analyze-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .analyze-progress {
    width: 100%;
    max-width: 400px;
  }

  .progress-stage {
    text-align: center;
    font-size: 0.875rem;
    color: #9ca3af;
    margin: 0.5rem 0 0;
  }

  .error {
    color: #ef4444;
    font-size: 0.875rem;
    text-align: center;
  }

  .workspace-footer {
    display: flex;
    justify-content: center;
    padding-top: 1.5rem;
    border-top: 1px solid #374151;
    margin-top: 1.5rem;
  }
</style>
