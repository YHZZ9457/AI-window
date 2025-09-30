<script lang="ts">
  import { onMount } from 'svelte';
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeTextFile } from '@tauri-apps/plugin-fs';
  import { theme } from '$lib/stores/theme';
  import { _ } from 'svelte-i18n';

  let appWindow: WebviewWindow | null = null;

  type Message = {
    role: 'user' | 'assistant' | 'system';
    content: string;
  };

  let prompt = $state('');
  let messages = $state<Message[]>([
    { role: 'assistant', content: $_('home.initialMessage') }
  ]);
  let isLoading = $state(false);
  let outputAreaElement: HTMLElement;

  function scrollToBottom() {
    if (outputAreaElement) {
      outputAreaElement.scrollTop = outputAreaElement.scrollHeight;
    }
  }

  $effect(() => {
    if (messages) {
      scrollToBottom();
    }
  });

  async function handleSubmit() {
    if (!prompt || isLoading) return;

    const userMessage: Message = { role: 'user', content: prompt };
    prompt = '';

    if (messages.length === 1 && messages[0].content === $_('home.initialMessage')) {
      messages = [userMessage];
    } else {
      messages = [...messages, userMessage];
    }

    isLoading = true;

    try {
      const result = await invoke('ask_ai', { messages });
      messages = [...messages, { role: 'assistant', content: result as string }];
    } catch (error) {
      messages = [...messages, { role: 'assistant', content: `Error: ${error}` }];
    } finally {
      isLoading = false;
    }
  }

  function clearChat() {
    messages = [{ role: 'assistant', content: $_('home.initialMessage') }];
  }

  function toggleTheme() {
    theme.update(current => {
      if (current === 'light') return 'dark';
      if (current === 'dark') return 'light';
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'light' : 'dark';
    });
  }

  async function exportToJSON() {
    const chatData = {
      exportDate: new Date().toISOString(),
      messageCount: messages.length,
      messages: messages
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
    };
    document.addEventListener('keydown', handleKey);
    scrollToBottom();

    return () => {
      document.removeEventListener('keydown', handleKey);
    };
  });
</script>

<main data-tauri-drag-region class="glass">
  <div class="header">
    <div class="header-left">
      <svg class="logo" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
        <path d="M2 17l10 5 10-5"></path>
        <path d="M2 12l10 5 10-5"></path>
      </svg>
      <h1 class="title">{$_('home.title')}</h1>
    </div>
    <div class="header-buttons">
      <button onclick={clearChat} class="header-button" aria-label={$_('home.buttons.clear')} title={$_('home.buttons.clear')}>
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 6 5 6 21 6"></polyline>
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
          <line x1="10" y1="11" x2="10" y2="17"></line>
          <line x1="14" y1="11" x2="14" y2="17"></line>
        </svg>
      </button>
      <button onclick={exportToJSON} class="header-button" aria-label={$_('home.buttons.export')} title={$_('home.buttons.export')}>
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
          <polyline points="7 10 12 15 17 10"></polyline>
          <line x1="12" y1="15" x2="12" y2="3"></line>
        </svg>
      </button>
      <button onclick={toggleTheme} class="header-button" title={$_('home.buttons.theme')} aria-label={$_('home.buttons.theme')}>
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="5"></circle>
          <line x1="12" y1="1" x2="12" y2="3"></line>
          <line x1="12" y1="21" x2="12" y2="23"></line>
          <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
          <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
          <line x1="1" y1="12" x2="3" y2="12"></line>
          <line x1="21" y1="12" x2="23" y2="12"></line>
          <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
          <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
        </svg>
      </button>
      <a href="/settings" class="header-button" title={$_('home.buttons.settings')} aria-label={$_('home.buttons.settings')}>
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0-.33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </a>
    </div>
  </div>
  
  <div class="chat-container">
    <div class="messages-container glass-light" bind:this={outputAreaElement}>
      {#each messages as message, i (i)}
        <div class="message" class:user={message.role === 'user'} class:assistant={message.role === 'assistant'}>
          <div class="message-header">
            <div class="role-icon">
              {#if message.role === 'user'}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                  <circle cx="12" cy="7" r="4"></circle>
                </svg>
              {:else}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"></path>
                </svg>
              {/if}
            </div>
            <span class="role">{message.role === 'user' ? $_('home.you') : $_('home.ai')}</span>
            <span class="timestamp">{new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}</span>
          </div>
          <div class="content">{message.content}</div>
        </div>
      {/each}
      {#if isLoading}
        <div class="message assistant loading-message">
          <div class="message-header">
            <div class="role-icon">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"></path>
              </svg>
            </div>
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
          placeholder={$_('home.placeholder')} 
          bind:value={prompt}
          onkeydown={(e) => e.key === 'Enter' && handleSubmit()}
          disabled={isLoading}
        />
        <button onclick={handleSubmit} disabled={isLoading} class="send-button">
          {#if isLoading}
            <svg class="loading-icon" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 12a9 9 0 11-6.219-8.56"></path>
            </svg>
          {:else}
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="22" y1="2" x2="11" y2="13"></line>
              <polygon points="22,2 15,22 11,13 2,9"></polygon>
            </svg>
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
