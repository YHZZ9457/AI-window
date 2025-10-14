<script lang="ts">
  import '@fontsource/inter';
  import '../app.css';
  import { theme, applyTheme, watchSystemTheme } from '$lib/stores/theme';
  import { onMount } from 'svelte';
  import { _, isLoading } from 'svelte-i18n';
  import { chat } from '$lib/stores/chat.store';
  import { clearChatShortcut, borderless, minimalMode, minimalModeShortcut } from '$lib/stores/settings.store';
  import { invoke } from '@tauri-apps/api/core';

  let { children } = $props();

  $effect(() => {
    if (!$isLoading) {
      // Use clearChat to ensure the translated message is always set after i18n loads.
      chat.clearChat($_('home.initialMessage'));
    }
  });

  onMount(() => {
    // Apply initial theme
    const unsubscribeTheme = theme.subscribe(currentTheme => {
      applyTheme(currentTheme);
    });

    // Watch for system theme changes
    const cleanupTheme = watchSystemTheme();

    return () => {
      unsubscribeTheme();
      cleanupTheme();
    };
  });

  $effect(() => {
    if ($borderless !== undefined) {
      invoke('set_decorations', { decorations: !$borderless });
    }
  });

  // $effect(() => {
  //   if ($minimalMode !== undefined) {
  //     invoke('set_minimal_mode', { minimal: $minimalMode });
  //     console.log('Minimal mode:', $minimalMode);
  //   }
  // });

  function handleKeyDown(event: KeyboardEvent) {
    const currentShortcut = getCurrentShortcut(event);
    if (currentShortcut === $clearChatShortcut) {
      event.preventDefault();
      chat.clearChat($_('home.initialMessage'));
    }
    // } else if (currentShortcut === $minimalModeShortcut) {
    //   event.preventDefault();
    //   minimalMode.set(!$minimalMode);
    // }
  }

  function getCurrentShortcut(e: KeyboardEvent): string {
    const parts = [];
    if (e.ctrlKey || e.metaKey) parts.push('Ctrl');
    if (e.altKey) parts.push('Alt');
    if (e.shiftKey) parts.push('Shift');
    if (e.metaKey && !e.ctrlKey) parts.push('Super');

    // 安全地处理 key 属性
    if (!e.key) return parts.join('+');
    
    const key = e.key.toUpperCase();
    let finalKey = key;
    if (key.startsWith('ARROW')) {
      finalKey = key.substring(5);
    }
    if (finalKey === ' ') {
      finalKey = 'SPACE';
    }
    
    if (!['CONTROL', 'ALT', 'SHIFT', 'META'].includes(finalKey)) {
        parts.push(finalKey);
    }
    
    return parts.join('+');
  }
</script>

<svelte:window on:keydown={handleKeyDown} />

<svelte:head>
  {#if !$isLoading}
    <title>{$_('app.title')}</title>
  {/if}

</svelte:head>

<div class="app" data-tauri-drag-region>
  {#if !$isLoading}
    {@render children()}
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
