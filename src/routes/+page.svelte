<script lang="ts">
  import { onMount } from 'svelte';
  import { Header, MainContent, SettingsModal, LicenseModal } from '$lib/components';
  import { ToastContainer } from '$lib/components/shared';
  import { licenseStore, settingsStore, uiStore } from '$lib/stores';
  import { getLicenseStatus, getSettings } from '$lib/utils/tauri';

  let showSettingsModal = $state(false);
  let showLicenseModal = $state(false);

  onMount(async () => {
    // Load initial state from backend
    try {
      const [license, settings] = await Promise.all([
        getLicenseStatus(),
        getSettings()
      ]);
      
      licenseStore.setLicense(license);
      settingsStore.loadSettings(settings);
    } catch (e) {
      console.error('Failed to load initial state:', e);
      uiStore.error('Failed to load settings');
    }
  });

  function handleSettingsClick() {
    showSettingsModal = true;
  }

  function handleLicenseClick() {
    showLicenseModal = true;
  }
</script>

<div class="app">
  <Header 
    onSettingsClick={handleSettingsClick}
    onLicenseClick={handleLicenseClick}
  />
  <MainContent />
</div>

<ToastContainer />

{#if showSettingsModal}
  <SettingsModal onClose={() => showSettingsModal = false} />
{/if}

{#if showLicenseModal}
  <LicenseModal onClose={() => showLicenseModal = false} />
{/if}

<style>
  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    background: #111827;
    color: #f3f4f6;
    min-height: 100vh;
    overflow: hidden;
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }

  /* Global button reset */
  :global(button) {
    font-family: inherit;
  }

  /* Scrollbar styling */
  :global(::-webkit-scrollbar) {
    width: 8px;
    height: 8px;
  }

  :global(::-webkit-scrollbar-track) {
    background: #1f2937;
  }

  :global(::-webkit-scrollbar-thumb) {
    background: #4b5563;
    border-radius: 4px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: #6b7280;
  }
</style>
