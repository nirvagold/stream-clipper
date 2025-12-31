<script lang="ts">
  import { projectStore } from '$lib/stores';
  import ImportView from './ImportView.svelte';
  import WorkspaceView from './WorkspaceView.svelte';

  const hasVideo = $derived($projectStore.videoPath !== null);
</script>

<main class="main-content">
  {#if hasVideo}
    <WorkspaceView />
  {:else}
    <div class="import-container">
      <div class="welcome">
        <h1>🎬 Stream Clipper</h1>
        <p>Auto-detect highlight moments from your streaming videos</p>
      </div>
      <ImportView />
    </div>
  {/if}
</main>

<style>
  .main-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .import-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
  }

  .welcome {
    text-align: center;
    margin-bottom: 2rem;
  }

  .welcome h1 {
    font-size: 2rem;
    font-weight: 700;
    color: white;
    margin: 0 0 0.5rem;
  }

  .welcome p {
    font-size: 1rem;
    color: #9ca3af;
    margin: 0;
  }
</style>
