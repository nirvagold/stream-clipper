<script lang="ts">
  import VideoDropzone from './VideoDropzone.svelte';
  import ChatDropzone from './ChatDropzone.svelte';
</script>

<div class="import-view">
  <VideoDropzone />
  <ChatDropzone />
</div>

<style>
  .import-view {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 600px;
    margin: 0 auto;
    padding: 2rem;
  }
</style>
