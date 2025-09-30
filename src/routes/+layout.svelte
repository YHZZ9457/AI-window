<script>
  import '../app.css';
  import { theme, applyTheme, watchSystemTheme } from '$lib/stores/theme';
  import { onMount } from 'svelte';
  import { _, isLoading } from 'svelte-i18n';

  onMount(() => {
    // Apply initial theme
    const unsubscribe = theme.subscribe(currentTheme => {
      applyTheme(currentTheme);
    });

    // Watch for system theme changes
    const cleanup = watchSystemTheme();

    return () => {
      unsubscribe();
      cleanup();
    };
  });
</script>

<svelte:head>
  {#if !$isLoading}
    <title>{$_('app.title')}</title>
  {/if}
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="">
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet">
</svelte:head>

<div class="app">
  {#if !$isLoading}
    <slot />
  {:else}
    <!-- You can add a loading spinner here if you want -->
  {/if}
</div>

<style>
  .app {
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }
</style>
