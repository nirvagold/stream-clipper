<script lang="ts">
  import { licenseStore } from '$lib/stores';
  import { ProBadge } from './shared';

  interface Props {
    onSettingsClick?: () => void;
    onLicenseClick?: () => void;
  }

  let { onSettingsClick, onLicenseClick }: Props = $props();
  
  const isPro = $derived($licenseStore.isPro);
</script>

<header class="header">
  <div class="logo">
    <span class="logo-icon">🎬</span>
    <span class="logo-text">Stream Clipper</span>
  </div>
  
  <div class="header-actions">
    <button class="header-btn" onclick={onSettingsClick} title="Settings">
      ⚙️
    </button>
    
    <button 
      class="license-btn" 
      class:is-pro={isPro}
      onclick={onLicenseClick}
      title={isPro ? 'Pro License Active' : 'Upgrade to Pro'}
    >
      {#if isPro}
        <span class="crown">👑</span>
        <span>Pro</span>
      {:else}
        <span>Upgrade</span>
      {/if}
    </button>
  </div>
</header>

<style>
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1.5rem;
    background: #1f2937;
    border-bottom: 1px solid #374151;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .logo-icon {
    font-size: 1.5rem;
  }

  .logo-text {
    font-size: 1.25rem;
    font-weight: 600;
    color: white;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .header-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    background: transparent;
    border: none;
    border-radius: 8px;
    font-size: 1.25rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .header-btn:hover {
    background: #374151;
  }

  .license-btn {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 1rem;
    background: #374151;
    border: none;
    border-radius: 8px;
    color: #d1d5db;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .license-btn:hover {
    background: #4b5563;
  }

  .license-btn.is-pro {
    background: linear-gradient(135deg, #f59e0b, #d97706);
    color: white;
  }

  .crown {
    font-size: 1rem;
  }
</style>
