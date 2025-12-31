// UI store - toast notifications and modal state
import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration: number;
}

export interface UIState {
  toasts: Toast[];
  settingsModalOpen: boolean;
  licenseModalOpen: boolean;
  previewModalOpen: boolean;
  keywordEditorOpen: boolean;
}

const initialState: UIState = {
  toasts: [],
  settingsModalOpen: false,
  licenseModalOpen: false,
  previewModalOpen: false,
  keywordEditorOpen: false,
};

function createUIStore() {
  const { subscribe, update } = writable<UIState>(initialState);

  let toastId = 0;

  return {
    subscribe,
    
    // Toast methods
    showToast: (type: ToastType, message: string, duration = 5000) => {
      const id = `toast-${++toastId}`;
      update(s => ({
        ...s,
        toasts: [...s.toasts, { id, type, message, duration }],
      }));
      
      // Auto-remove after duration
      if (duration > 0) {
        setTimeout(() => {
          update(s => ({
            ...s,
            toasts: s.toasts.filter(t => t.id !== id),
          }));
        }, duration);
      }
      
      return id;
    },
    
    removeToast: (id: string) => update(s => ({
      ...s,
      toasts: s.toasts.filter(t => t.id !== id),
    })),
    
    // Convenience methods
    success: (message: string) => {
      const store = createUIStore();
      update(s => {
        const id = `toast-${++toastId}`;
        setTimeout(() => {
          update(st => ({ ...st, toasts: st.toasts.filter(t => t.id !== id) }));
        }, 5000);
        return { ...s, toasts: [...s.toasts, { id, type: 'success' as ToastType, message, duration: 5000 }] };
      });
    },
    
    error: (message: string) => {
      update(s => {
        const id = `toast-${++toastId}`;
        setTimeout(() => {
          update(st => ({ ...st, toasts: st.toasts.filter(t => t.id !== id) }));
        }, 7000);
        return { ...s, toasts: [...s.toasts, { id, type: 'error' as ToastType, message, duration: 7000 }] };
      });
    },
    
    warning: (message: string) => {
      update(s => {
        const id = `toast-${++toastId}`;
        setTimeout(() => {
          update(st => ({ ...st, toasts: st.toasts.filter(t => t.id !== id) }));
        }, 5000);
        return { ...s, toasts: [...s.toasts, { id, type: 'warning' as ToastType, message, duration: 5000 }] };
      });
    },
    
    info: (message: string) => {
      update(s => {
        const id = `toast-${++toastId}`;
        setTimeout(() => {
          update(st => ({ ...st, toasts: st.toasts.filter(t => t.id !== id) }));
        }, 4000);
        return { ...s, toasts: [...s.toasts, { id, type: 'info' as ToastType, message, duration: 4000 }] };
      });
    },
    
    // Modal methods
    openSettingsModal: () => update(s => ({ ...s, settingsModalOpen: true })),
    closeSettingsModal: () => update(s => ({ ...s, settingsModalOpen: false })),
    
    openLicenseModal: () => update(s => ({ ...s, licenseModalOpen: true })),
    closeLicenseModal: () => update(s => ({ ...s, licenseModalOpen: false })),
    
    openPreviewModal: () => update(s => ({ ...s, previewModalOpen: true })),
    closePreviewModal: () => update(s => ({ ...s, previewModalOpen: false })),
    
    openKeywordEditor: () => update(s => ({ ...s, keywordEditorOpen: true })),
    closeKeywordEditor: () => update(s => ({ ...s, keywordEditorOpen: false })),
  };
}

export const uiStore = createUIStore();
