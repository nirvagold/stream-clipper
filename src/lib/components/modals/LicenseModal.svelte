<script lang="ts">
  import { licenseStore, PRO_FEATURES } from '$lib/stores';
  import { activateLicense, deactivateLicense } from '$lib/utils/tauri';
  import { Button } from '../shared';

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  const license = $derived($licenseStore);
  
  let licenseKey = $state('');
  let isLoading = $state(false);
  let error = $state<string | null>(null);
  let success = $state<string | null>(null);

  async function handleActivate() {
    if (!licenseKey.trim()) {
      error = 'Please enter a license key';
      return;
    }

    isLoading = true;
    error = null;
    success = null;

    try {
      const result = await activateLicense(licenseKey.trim());
      licenseStore.setLicense(result);
      success = 'License activated successfully!';
      licenseKey = '';
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to activate license';
    } finally {
      isLoading = false;
    }
  }

  async function handleDeactivate() {
    isLoading = true;
    error = null;

    try {
      await deactivateLicense();
      licenseStore.clearLicense();
      success = 'License deactivated';
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to deactivate';
    } finally {
      isLoading = false;
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
      <h2>{license.isPro ? '👑 Pro License' : '🚀 Upgrade to Pro'}</h2>
      <button class="close-btn" onclick={onClose}>✕</button>
    </div>

    <div class="modal-body">
      {#if license.isPro}
        <div class="pro-status">
          <div class="status-badge">✓ Pro Active</div>
          <p class="license-info">
            Key: {license.licenseKey?.slice(0, 5)}...{license.licenseKey?.slice(-5)}
          </p>
          {#if license.activatedAt}
            <p class="activated-date">Activated: {new Date(license.activatedAt).toLocaleDateString()}</p>
          {/if}
        </div>

        <Button variant="secondary" onclick={handleDeactivate} loading={isLoading}>
          Deactivate License
        </Button>
      {:else}
        <div class="features-list">
          <h3>Pro Features:</h3>
          <ul>
            {#each PRO_FEATURES as feature}
              <li>✓ {feature.name}</li>
            {/each}
          </ul>
        </div>

        <div class="activate-form">
          <label for="license-key">License Key:</label>
          <input
            id="license-key"
            type="text"
            bind:value={licenseKey}
            placeholder="XXXXX-XXXXX-XXXXX-XXXXX"
            disabled={isLoading}
          />
          <Button variant="primary" onclick={handleActivate} loading={isLoading}>
            Activate
          </Button>
        </div>

        <p class="purchase-link">
          Don't have a key? <a href="https://streamclipper.app/buy" target="_blank">Purchase Pro</a>
        </p>
      {/if}

      {#if error}
        <p class="error">{error}</p>
      {/if}
      {#if success}
        <p class="success">{success}</p>
      {/if}
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
    max-width: 420px;
    max-height: 90vh;
    overflow-y: auto;
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
  }

  .pro-status {
    text-align: center;
    margin-bottom: 1.5rem;
  }

  .status-badge {
    display: inline-block;
    background: linear-gradient(135deg, #f59e0b, #d97706);
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 9999px;
    font-weight: 600;
    margin-bottom: 1rem;
  }

  .license-info, .activated-date {
    font-size: 0.875rem;
    color: #9ca3af;
    margin: 0.25rem 0;
  }

  .features-list {
    margin-bottom: 1.5rem;
  }

  .features-list h3 {
    font-size: 0.875rem;
    color: #d1d5db;
    margin: 0 0 0.75rem;
  }

  .features-list ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .features-list li {
    font-size: 0.875rem;
    color: #10b981;
    padding: 0.375rem 0;
  }

  .activate-form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .activate-form label {
    font-size: 0.875rem;
    color: #d1d5db;
  }

  .activate-form input {
    padding: 0.625rem 0.875rem;
    background: #374151;
    border: 1px solid #4b5563;
    border-radius: 6px;
    color: #f3f4f6;
    font-size: 0.875rem;
    font-family: monospace;
    text-transform: uppercase;
  }

  .activate-form input:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .purchase-link {
    text-align: center;
    font-size: 0.875rem;
    color: #9ca3af;
    margin-top: 1rem;
  }

  .purchase-link a {
    color: #3b82f6;
    text-decoration: none;
  }

  .purchase-link a:hover {
    text-decoration: underline;
  }

  .error {
    color: #ef4444;
    font-size: 0.875rem;
    margin-top: 1rem;
    text-align: center;
  }

  .success {
    color: #10b981;
    font-size: 0.875rem;
    margin-top: 1rem;
    text-align: center;
  }
</style>
