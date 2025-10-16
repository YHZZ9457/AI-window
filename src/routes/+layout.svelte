<script lang="ts">
  import '@fontsource/inter';
  import '../app.css';
  import { theme, applyTheme, watchSystemTheme } from '$lib/stores/theme';
  import { onMount } from 'svelte';
  import { _, isLoading } from 'svelte-i18n';
  import { chat } from '$lib/stores/chat.store';
  import { clearChatShortcut, borderless } from '$lib/stores/settings.store';
  import { invoke } from '@tauri-apps/api/core';

  let { children } = $props();

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

  function handleKeyDown(event: KeyboardEvent) {
    const currentShortcut = getCurrentShortcut(event);
    if (currentShortcut === $clearChatShortcut) {
      event.preventDefault();
      chat.clearChat();
    }
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
      finalKey = 'Space';
    }
    
    if (!['Control', 'Alt', 'Shift', 'Meta'].includes(finalKey)) {
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