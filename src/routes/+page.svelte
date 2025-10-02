<script lang="ts">
  import SendIcon from '$lib/components/icons/SendIcon.svelte';
  import LoadingIcon from '$lib/components/icons/LoadingIcon.svelte';
  import AiIcon from '$lib/components/icons/AiIcon.svelte';
  import UserIcon from '$lib/components/icons/UserIcon.svelte';
  import SettingsIcon from '$lib/components/icons/SettingsIcon.svelte';
  import LanguageIcon from '$lib/components/icons/LanguageIcon.svelte';
  import ThemeIcon from '$lib/components/icons/ThemeIcon.svelte';
  import ExportIcon from '$lib/components/icons/ExportIcon.svelte';
  import ClearIcon from '$lib/components/icons/ClearIcon.svelte';
  import LogoIcon from '$lib/components/icons/LogoIcon.svelte';
  import { onMount, onDestroy } from 'svelte';
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeTextFile } from '@tauri-apps/plugin-fs';
  import { theme } from '$lib/stores/theme';
  import { _, locale } from 'svelte-i18n';
  import { chat } from '$lib/stores/chat.store';
  import { clearChatShortcut } from '$lib/stores/settings.store';

  let appWindow: WebviewWindow | null = null;

  type Message = {
    role: 'user' | 'assistant' | 'system';
    content: string;
  };

  let prompt = $state('');
  let isLoading = $state(false);
  let outputAreaElement: HTMLElement;
  let showLanguageMenu = $state(false);
  let languageMenuElement: HTMLElement;

  function scrollToBottom() {
    if (outputAreaElement) {
      outputAreaElement.scrollTop = outputAreaElement.scrollHeight;
    }
  }

  $effect(() => {
    if ($chat) {
      scrollToBottom();
    }
  });

  async function handleSubmit() {
    if (!prompt || isLoading) return;

    const userMessageContent = prompt;
    prompt = '';
    chat.addUserMessage(userMessageContent);

    isLoading = true;

    try {
      const result = await invoke('ask_ai', { messages: $chat });
      chat.addAssistantMessage(result as string);
    } catch (error) {
      chat.addAssistantMessage(`Error: ${error}`);
    } finally {
      isLoading = false;
    }
  }

  function toggleTheme() {
    theme.update(current => {
      if (current === 'light') return 'dark';
      if (current === 'dark') return 'light';
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'light' : 'dark';
    });
  }

  function setLanguage(lang: string) {
    locale.set(lang);
    showLanguageMenu = false;
  }

  async function exportToJSON() {
    const chatData = {
      exportDate: new Date().toISOString(),
      messageCount: $chat.length,
      messages: $chat
    };
    
    const jsonContent = JSON.stringify(chatData, null, 2);

    try {
      const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
      const defaultFileName = `chat-export-${timestamp}.json`;
      
      const filePath = await save({
        defaultPath: defaultFileName,
        filters: [{
          name: 'JSON Files',
          extensions: ['json']
        }]
      });

      if (filePath) {
        await writeTextFile(filePath, jsonContent);
      }
    } catch (error) {
      console.error('Failed to save file:', error);
    }
  }

  onMount(() => {
    WebviewWindow.getByLabel('main').then(window => {
      appWindow = window;
    });
    
    const handleKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && appWindow) {
        appWindow.hide();
      }
      
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'E') {
        e.preventDefault();
        exportToJSON();
      }
      
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'T') {
        e.preventDefault();
        toggleTheme();
      }
      
      if ((e.ctrlKey || e.metaKey) && e.key === '/') {
        e.preventDefault();
        const input = document.querySelector('.message-input') as HTMLInputElement;
        if (input) {
          input.focus();
        }
      }
    };

    const handleClickOutside = (event: MouseEvent) => {
        if (languageMenuElement && !languageMenuElement.contains(event.target as Node)) {
            showLanguageMenu = false;
        }
    };

    document.addEventListener('keydown', handleKey);
    document.addEventListener('click', handleClickOutside, true);
    scrollToBottom();

    return () => {
      document.removeEventListener('keydown', handleKey);
      document.removeEventListener('click', handleClickOutside, true);
    };
  });
</script>

<main data-tauri-drag-region class="glass">
  <div class="header">
    <div class="header-left">
      <LogoIcon />
      <h1 class="title">{$_('home.title')}</h1>
    </div>
    <div class="header-buttons">
      <button onclick={() => chat.clearChat($_('home.initialMessage'))} class="header-button" aria-label={$_('home.buttons.clear')} title={`${$_('home.buttons.clear')} (${$clearChatShortcut})`}>
        <ClearIcon />
      </button>
      <button onclick={exportToJSON} class="header-button" aria-label={$_('home.buttons.export')} title={`${$_('home.buttons.export')} (Ctrl+Shift+E)`}>
        <ExportIcon />
      </button>
      <button onclick={toggleTheme} class="header-button" title={`${$_('home.buttons.theme')} (Ctrl+Shift+T)`} aria-label={$_('home.buttons.theme')}>
        <ThemeIcon />
      </button>

      <div class="language-menu-container" bind:this={languageMenuElement}>
        <button onclick={() => showLanguageMenu = !showLanguageMenu} class="header-button" title={$_('home.buttons.language')} aria-label={$_('home.buttons.language')}>
            <LanguageIcon />
        </button>
        {#if showLanguageMenu}
            <div class="language-menu">
                <button class:active={$locale === 'en'} onclick={() => setLanguage('en')}>English</button>
                <button class:active={$locale === 'zh-CN'} onclick={() => setLanguage('zh-CN')}>简体中文</button>
                <button class:active={$locale === 'zh-TW'} onclick={() => setLanguage('zh-TW')}>繁體中文</button>
                <button class:active={$locale === 'ja'} onclick={() => setLanguage('ja')}>日本語</button>
            </div>
        {/if}
      </div>

      <a href="/settings" class="header-button" title={$_('home.buttons.settings')} aria-label={$_('home.buttons.settings')}>
        <SettingsIcon />
      </a>
    </div>
  </div>
  
  <div class="chat-container">
    <div class="messages-container glass-light" bind:this={outputAreaElement}>
      {#each $chat as message, i (i)}
        <div class="message" class:user={message.role === 'user'} class:assistant={message.role === 'assistant'}>
          <div class="message-header">
            <div class="role-icon">
              {#if message.role === 'user'}
                <UserIcon />
              {:else}
                                <AiIcon />
                              {:else}
                                <AiIcon />            </div>
            <span class="role">{$_('home.ai')}</span>
          </div>
          <div class="content">
            <div class="typing-indicator">
              <span></span>
              <span></span>
              <span></span>
            </div>
          </div>
        </div>
      {/if}
    </div>
    
    <div class="input-container">
      <div class="input-wrapper">
        <input 
          type="text" 
          class="message-input"
          placeholder={`${$_('home.placeholder')} (Ctrl+/ to focus)`} 
          bind:value={prompt}
          onkeydown={(e) => e.key === 'Enter' && handleSubmit()}
          disabled={isLoading}
        />
        <button onclick={handleSubmit} disabled={isLoading} class="send-button">
          {#if isLoading}
            <LoadingIcon />
          {:else}
            <SendIcon />
          {/if}
        </button>
      </div>
    </div>
  </div>
</main>

<style>
  main {
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    font-family: var(--font-family);
    background: var(--bg-primary);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-md) var(--spacing-lg);
    background: var(--bg-primary);
    border-bottom: 1px solid var(--border-primary);
    flex-shrink: 0;
    min-height: 60px;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: var(--spacing-md);
  }

  .logo {
    width: 32px;
    height: 32px;
    color: var(--text-primary);
    opacity: 0.9;
  }

  .title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    margin: 0;
    color: var(--text-primary);
  }

  .header-buttons {
    display: flex;
    gap: var(--spacing-sm);
    align-items: center;
  }

  .header-button {
    width: 36px;
    height: 36px;
    border-radius: var(--radius-full);
    background: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    color: var(--text-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: var(--transition-normal);
    text-decoration: none;
  }

  .header-button:hover {
    background: var(--bg-tertiary);
    transform: translateY(-2px);
    box-shadow: var(--shadow-soft);
  }

  .header-button:active {
    transform: translateY(0);
  }

  .language-menu-container {
      position: relative;
  }

  .language-menu {
      position: absolute;
      top: calc(100% + var(--spacing-sm));
      right: 0;
      background: var(--bg-secondary);
      border: 1px solid var(--border-primary);
      border-radius: var(--radius-md);
      padding: var(--spacing-sm);
      display: flex;
      flex-direction: column;
      gap: var(--spacing-xs);
      z-index: var(--z-dropdown);
      box-shadow: var(--shadow-soft);
      width: max-content;
  }

  .language-menu button {
      background: none;
      border: none;
      color: var(--text-secondary);
      padding: var(--spacing-sm) var(--spacing-md);
      border-radius: var(--radius-sm);
      cursor: pointer;
      text-align: left;
      width: 100%;
      font-size: var(--font-size-sm);
  }

  .language-menu button:hover {
      background: var(--bg-tertiary);
      color: var(--text-primary);
  }
  .language-menu button.active {
      color: var(--text-primary);
      font-weight: var(--font-weight-medium);
  }

  .chat-container {
    padding: var(--spacing-md);
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    overflow: hidden;
    gap: var(--spacing-md);
  }

  .messages-container {
    flex-grow: 1;
    background: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-lg);
    padding: var(--spacing-lg);
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    box-shadow: var(--shadow-soft);
  }

  .messages-container::-webkit-scrollbar {
    width: 4px;
  }

  .messages-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .messages-container::-webkit-scrollbar-thumb {
    background: var(--border-primary);
    border-radius: var(--radius-full);
  }

  .messages-container::-webkit-scrollbar-thumb:hover {
    background: var(--text-muted);
  }

  .message {
    padding: var(--spacing-sm) var(--spacing-md);
    border-radius: var(--radius-md);
    max-width: 85%;
    width: fit-content;
    animation: fadeIn 0.3s ease-out;
    border: 1px solid var(--border-primary);
    box-shadow: 0 1px 2px var(--shadow-soft);
  }

  .message.assistant {
    background: var(--bg-primary);
    color: var(--text-primary);
    align-self: flex-start;
    border-bottom-left-radius: var(--radius-sm);
    transition: var(--transition-normal);
  }

  .message.assistant:hover {
    background: var(--bg-secondary);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .message.user {
    background: var(--blue-primary);
    color: white;
    align-self: flex-end;
    border-bottom-right-radius: var(--radius-sm);
    border-color: var(--blue-hover);
    transition: var(--transition-normal);
  }

  .message.user:hover {
    background: var(--blue-hover);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }

  .message-header {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    margin-bottom: var(--spacing-xs);
    font-size: var(--font-size-xs);
    opacity: 0.7;
  }

  .role-icon {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .role {
    font-weight: var(--font-weight-semibold);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .timestamp {
    font-size: var(--font-size-xs);
    opacity: 0.6;
    margin-left: auto;
  }

  .content {
    white-space: pre-wrap;
    line-height: 1.5;
    font-size: var(--font-size-sm);
  }

  .loading-message {
    opacity: 0.8;
  }

  .typing-indicator {
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .typing-indicator span {
    width: 8px;
    height: 8px;
    background: var(--text-muted);
    border-radius: 50%;
    animation: typing 1.4s infinite ease-in-out;
  }

  .typing-indicator span:nth-child(2) {
    animation-delay: 0.2s;
  }

  .typing-indicator span:nth-child(3) {
    animation-delay: 0.4s;
  }

  @keyframes typing {
    0%, 60%, 100% {
      transform: translateY(0);
    }
    30% {
      transform: translateY(-4px);
    }
  }

  .input-container {
    flex-shrink: 0;
  }

  .input-wrapper {
    display: flex;
    gap: var(--spacing-sm);
    align-items: center;
    background: var(--bg-primary);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-full);
    padding: var(--spacing-xs) var(--spacing-sm);
    box-shadow: 0 1px 4px var(--shadow-soft);
  }

  .message-input {
    flex-grow: 1;
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    outline: none;
    font-family: inherit;
  }

  .message-input::placeholder {
    color: var(--text-muted);
  }

  .message-input:focus {
    outline: none;
  }

  .send-button {
    width: 40px;
    height: 40px;
    border-radius: var(--radius-full);
    background: var(--primary);
    border: none;
    color: var(--bg-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: var(--transition-normal);
    flex-shrink: 0;
  }

  .send-button:hover:not(:disabled) {
    transform: scale(1.05);
    box-shadow: 0 8px 25px var(--shadow-glow);
    background: var(--primary-hover);
  }

  .send-button:active:not(:disabled) {
    transform: scale(0.95);
  }

  .send-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .loading-icon {
    animation: spin 1s linear infinite;
  }

  /* Responsive design */
  @media (max-width: 640px) {
    .header {
      padding: var(--spacing-md) var(--spacing-lg);
    }
    
    .title {
      font-size: var(--font-size-xl);
    }
    
    .chat-container {
      padding: var(--spacing-lg);
    }
    
    .message {
      max-width: 90%;
    }
    
    .messages-container {
      padding: var(--spacing-lg);
    }
  }
</style>