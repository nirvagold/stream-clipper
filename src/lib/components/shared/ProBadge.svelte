<script lang="ts">
  import { licenseStore } from '$lib/stores';

  interface Props {
    feature?: string;
    showWhenPro?: boolean;
  }

  let { feature = '', showWhenPro = false }: Props = $props();
  
  const isPro = $derived($licenseStore.isPro);
  const shouldShow = $derived(showWhenPro ? isPro : !isPro);
</script>

{#if shouldShow}
  <span class="pro-badge" title={feature ? `${feature} requires Pro` : 'Pro feature'}>
    PRO
  </span>
{/if}

<style>
  .pro-badge {
    display: inline-flex;
    align-items: center;
    padding: 0.125rem 0.375rem;
    font-size: 0.625rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    background: linear-gradient(135deg, #f59e0b, #d97706);
    color: white;
    border-radius: 4px;
  }
</style>
