// License store - from spec 2-design.md Section 2.2
import { writable, derived } from 'svelte/store';
import type { LicenseInfo } from '$lib/types';

export interface LicenseState {
  isPro: boolean;
  licenseKey: string | null;
  activatedAt: string | null;
  isValidating: boolean;
}

const initialState: LicenseState = {
  isPro: false,
  licenseKey: null,
  activatedAt: null,
  isValidating: false,
};

function createLicenseStore() {
  const { subscribe, set, update } = writable<LicenseState>(initialState);

  return {
    subscribe,
    setLicense: (info: LicenseInfo) => set({
      isPro: info.is_pro,
      licenseKey: info.license_key,
      activatedAt: info.activated_at,
      isValidating: false,
    }),
    clearLicense: () => set(initialState),
    startValidating: () => update(s => ({ ...s, isValidating: true })),
    stopValidating: () => update(s => ({ ...s, isValidating: false })),
    reset: () => set(initialState),
  };
}

export const licenseStore = createLicenseStore();

// Pro features list with display names
export const PRO_FEATURES = [
  { id: 'chat_detection', name: 'Chat Activity Detection' },
  { id: 'vertical_crop', name: 'Vertical Crop (9:16)' },
  { id: 'custom_keywords', name: 'Custom Keywords' },
  { id: 'unlimited_clips', name: 'Unlimited Clips' },
  { id: 'high_resolution', name: 'Up to 4K Resolution' },
  { id: 'no_watermark', name: 'No Watermark' },
  { id: 'batch_export', name: 'Batch Export' },
  { id: 'fade_effect', name: 'Fade In/Out Effect' },
] as const;

export type ProFeature = typeof PRO_FEATURES[number]['id'];

// Check if feature requires Pro
export function requiresPro(feature: ProFeature): boolean {
  return PRO_FEATURES.some(f => f.id === feature);
}
