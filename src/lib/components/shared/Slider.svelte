<script lang="ts">
  interface Props {
    value: number;
    min?: number;
    max?: number;
    step?: number;
    label?: string;
    id?: string;
    disabled?: boolean;
    onchange?: (value: number) => void;
  }

  let { 
    value = $bindable(0), 
    min = 0, 
    max = 100, 
    step = 1, 
    label = '',
    id = `slider-${Math.random().toString(36).slice(2)}`,
    disabled = false,
    onchange 
  }: Props = $props();

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    value = parseFloat(target.value);
    onchange?.(value);
  }
</script>

<div class="slider-container" class:disabled>
  {#if label}
    <label class="slider-label" for={id}>{label}</label>
  {/if}
  <div class="slider-row">
    <input
      {id}
      type="range"
      class="slider"
      {min}
      {max}
      {step}
      {value}
      {disabled}
      oninput={handleInput}
    />
    <span class="slider-value">{value.toFixed(1)}</span>
  </div>
</div>

<style>
  .slider-container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .slider-container.disabled {
    opacity: 0.5;
  }

  .slider-label {
    font-size: 0.875rem;
    color: #d1d5db;
  }

  .slider-row {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .slider {
    flex: 1;
    height: 6px;
    background: #374151;
    border-radius: 9999px;
    appearance: none;
    cursor: pointer;
  }

  .slider:disabled {
    cursor: not-allowed;
  }

  .slider::-webkit-slider-thumb {
    appearance: none;
    width: 18px;
    height: 18px;
    background: #3b82f6;
    border-radius: 50%;
    cursor: pointer;
    transition: transform 0.2s;
  }

  .slider::-webkit-slider-thumb:hover {
    transform: scale(1.1);
  }

  .slider::-moz-range-thumb {
    width: 18px;
    height: 18px;
    background: #3b82f6;
    border: none;
    border-radius: 50%;
    cursor: pointer;
  }

  .slider-value {
    min-width: 2.5rem;
    text-align: right;
    font-size: 0.875rem;
    color: #9ca3af;
    font-family: monospace;
  }
</style>
