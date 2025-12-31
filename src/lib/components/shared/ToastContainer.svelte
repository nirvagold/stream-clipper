<script lang="ts">
  import { uiStore } from '$lib/stores/ui';
  import Toast from './Toast.svelte';

  const toasts = $derived($uiStore.toasts);

  function handleClose(id: string) {
    uiStore.removeToast(id);
  }
</script>

<div class="toast-container" aria-live="polite">
  {#each toasts as toast (toast.id)}
    <Toast 
      id={toast.id}
      type={toast.type}
      message={toast.message}
      onClose={handleClose}
    />
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: 1rem;
    right: 1rem;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    pointer-events: none;
  }

  .toast-container :global(.toast) {
    pointer-events: auto;
  }
</style>
