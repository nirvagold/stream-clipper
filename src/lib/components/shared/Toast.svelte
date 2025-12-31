<script lang="ts">
  import type { ToastType } from '$lib/stores/ui';

  interface Props {
    id: string;
    type: ToastType;
    message: string;
    onClose: (id: string) => void;
  }

  let { id, type, message, onClose }: Props = $props();

  const icons: Record<ToastType, string> = {
    success: '✓',
    error: '✕',
    warning: '⚠',
    info: 'ℹ',
  };

  function handleClose() {
    onClose(id);
  }
</script>

<div class="toast toast-{type}" role="alert">
  <span class="toast-icon">{icons[type]}</span>
  <span class="toast-message">{message}</span>
  <button class="toast-close" onclick={handleClose} aria-label="Close">×</button>
</div>

<style>
  .toast {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.875rem 1rem;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    animation: slideIn 0.3s ease-out;
    max-width: 400px;
  }

  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  .toast-success {
    background: #065f46;
    border: 1px solid #10b981;
    color: #d1fae5;
  }

  .toast-error {
    background: #7f1d1d;
    border: 1px solid #ef4444;
    color: #fecaca;
  }

  .toast-warning {
    background: #78350f;
    border: 1px solid #f59e0b;
    color: #fef3c7;
  }

  .toast-info {
    background: #1e3a5f;
    border: 1px solid #3b82f6;
    color: #dbeafe;
  }

  .toast-icon {
    font-size: 1.125rem;
    font-weight: 700;
    flex-shrink: 0;
  }

  .toast-message {
    flex: 1;
    font-size: 0.875rem;
    line-height: 1.4;
  }

  .toast-close {
    background: none;
    border: none;
    color: inherit;
    font-size: 1.25rem;
    cursor: pointer;
    opacity: 0.7;
    padding: 0;
    line-height: 1;
    flex-shrink: 0;
  }

  .toast-close:hover {
    opacity: 1;
  }
</style>
