<script lang="ts">
  import {
    SendIcon,
    LoadingIcon,
    AiIcon,
    UserIcon,
    SettingsIcon,
    LanguageIcon,
    ThemeIcon,
    ExportIcon,
    ClearIcon,
    LogoIcon,
    PlusIcon,
    FileTextIcon
  } from '$lib/components/icons';
  import Markdown from '$lib/components/Markdown.svelte';
  import { onMount, onDestroy } from 'svelte';
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { invoke } from '@tauri-apps/api/core';
  import { save, open } from '@tauri-apps/plugin-dialog';
  import { writeTextFile, readFile } from '@tauri-apps/plugin-fs';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
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
  let attachedFileName = $state<string | null>(null);
  let attachedFileContent = $state<string | null>(null);
  let copiedMessageIndex = $state<number | null>(null);

  async function copy(text: string, index: number) {
    await writeText(text);
    copiedMessageIndex = index;
    setTimeout(() => {
      copiedMessageIndex = null;
    }, 2000);
  }

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

  async function processFile(file: { path?: string; file?: File }) {
    if (isLoading) return;

    let fileName: string;
    let fileData: Uint8Array;

    try {
      if (file.path) {
        fileName = file.path.split(/[\\/]/).pop() || file.path;
        fileData = await readFile(file.path);
      } else if (file.file) {
        fileName = file.file.name;
        const arrayBuffer = await file.file.arrayBuffer();
        fileData = new Uint8Array(arrayBuffer);
      } else {
        return;
      }

      const content = await invoke('extract_text', {
        bytes: Array.from(fileData),
        fileName
      });

      attachedFileName = fileName;
      attachedFileContent = content as string;

      const input = document.querySelector('.message-input') as HTMLInputElement;
      if (input) {
        input.focus();
      }
    } catch (error) {
      console.error('Failed to process file:', error);
      // Optionally, display an error message to the user
    }
  }

  async function handleAttachment() {
    if (isLoading) return;
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Supported Files',
            extensions: [
              'txt',
              'md',
              'json',
              'csv',
              'html',
              'css',
              'js',
              'ts',
              'py',
              'rs',
              'pdf',
              'docx'
            ]
          }
        ]
      });

      if (typeof selected === 'string') {
        await processFile({ path: selected });
      }
    } catch (error) {
      console.error('Failed to open file:', error);
    }
  }

  function removeAttachment() {
    attachedFileName = null;
    attachedFileContent = null;
  }

  async function handleSubmit() {
    if ((!prompt && !attachedFileContent) || isLoading) return;

    // 1. Add the message to the Svelte store for UI display
    chat.addUserMessage(prompt, attachedFileContent ? { name: attachedFileName!, content: attachedFileContent } : null);

    // 2. Create a deep copy of messages for the backend to avoid mutating the UI state
    const messagesForBackend = structuredClone($chat);
    const lastMessage = messagesForBackend[messagesForBackend.length - 1];

    // 3. In the copy, combine the text and file content for the AI
    if (lastMessage && lastMessage.role === 'user' && lastMessage.attachment) {
      const fileText = `--- Attached File: ${lastMessage.attachment.name} ---\n${lastMessage.attachment.content}`;
      lastMessage.content = lastMessage.content ? `${lastMessage.content}\n\n${fileText}` : fileText;
      delete lastMessage.attachment; // Clean up attachment from backend payload
    }

    prompt = '';
    removeAttachment();

    isLoading = true;

    try {
      // 4. Send the modified copy to the backend
      const result = await invoke('ask_ai', { messages: messagesForBackend });
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

    const handleFile = (file: File) => {
      const allowedExtensions = [
        '.txt',
        '.md',
        '.json',
        '.csv',
        '.html',
        '.css',
        '.js',
        '.ts',
        '.py',
        '.rs',
        '.pdf',
        '.docx'
      ];
      const fileExtension = `.${file.name.split('.').pop()?.toLowerCase()}`;

      if (!allowedExtensions.includes(fileExtension)) {
        console.warn('Unsupported file type:', file.type);
        // Optionally, show a user-facing error message here.
        return;
      }

      processFile({ file });
    };

    const handlePaste = (event: ClipboardEvent) => {
      const file = event.clipboardData?.files[0];
      if (file) {
        event.preventDefault();
        handleFile(file);
      }
    };

    const handleDragOver = (event: DragEvent) => {
      event.preventDefault(); // Necessary to allow drop
    };

    const handleDrop = (event: DragEvent) => {
      event.preventDefault();
      const file = event.dataTransfer?.files[0];
      if (file) {
        handleFile(file);
      }
    };

    document.addEventListener('keydown', handleKey);
    document.addEventListener('click', handleClickOutside, true);
    document.addEventListener('paste', handlePaste);
    document.addEventListener('dragover', handleDragOver);
    document.addEventListener('drop', handleDrop);
    scrollToBottom();

    return () => {
      document.removeEventListener('keydown', handleKey);
      document.removeEventListener('click', handleClickOutside, true);
      document.removeEventListener('paste', handlePaste);
      document.removeEventListener('dragover', handleDragOver);
      document.removeEventListener('drop', handleDrop);
    };
  });
</script>

<main data-tauri-drag-region class="glass">
  <div class="header" data-tauri-drag-region>
    <div class="header-left" data-tauri-drag-region>
      <LogoIcon class="logo" />
      <h1 class="title" data-tauri-drag-region>{$_('home.title')}</h1>
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
        <div class="message" class:user={message.role === 'user'} class:assistant={message.role === 'assistant'} onclick={() => copy(message.content, i)} onkeydown={(e) => e.key === 'Enter' && copy(message.content, i)} tabindex="0" role="button">
          {#if copiedMessageIndex === i}
            <div class="copied-toast">{$_('home.copySuccess')}</div>
          {/if}
          <div class="message-header">
            <div class="role-icon">
              {#if message.role === 'user'}
                <UserIcon />
              {:else}
                <AiIcon />
              {/if}
            </div>
            <span class="role">{message.role === 'user' ? $_('home.you') : $_('home.ai')}</span>
            <span class="timestamp">{new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}</span>
          </div>
          <div class="content">
            {#if message.content}
            {#if message.role === 'assistant'}
              <Markdown text={message.content} />
            {:else}
              <div class="message-text">{message.content}</div>
            {/if}
            {/if}
            {#if message.attachment}
              <div class="attachment-block">
                <div class="attachment-icon"><FileTextIcon /></div>
                <span class="attachment-name">{message.attachment.name}</span>
              </div>
            {/if}
          </div>
        </div>
      {/each}
      {#if isLoading}
        <div class="message assistant loading-message">
          <div class="message-header">
            <div class="role-icon">
              <AiIcon />
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
        <button onclick={handleAttachment} class="attachment-button" aria-label={$_('home.buttons.attach')} title={$_('home.buttons.attach')}>
          <PlusIcon />
        </button>
        {#if attachedFileName}
          <div class="filename-pill">
            <span class="pill-text" title={attachedFileName}>{attachedFileName}</span>
            <button onclick={removeAttachment} class="remove-pill-button" title="Remove file">
              <ClearIcon />
            </button>
          </div>
        {/if}
        <input 
          type="text" 
          class="message-input"
          placeholder={attachedFileName ? $_('home.placeholderFileAttached') : `${$_('home.placeholder')} (Ctrl+/ to focus)`} 
          bind:value={prompt}
          onkeydown={(e) => e.key === 'Enter' && handleSubmit()}
          disabled={isLoading}
        />
        <button onclick={handleSubmit} disabled={isLoading} class="send-button">
          {#if isLoading}
            <LoadingIcon class="loading-icon" />
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
    align-items: center;
    justify-content: space-between;
    padding: var(--spacing-md) var(--spacing-lg);
    border-bottom: 1px solid var(--border-primary);
    flex-shrink: 0;
    min-height: 60px;
    -webkit-app-region: drag;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: var(--spacing-md);
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
    -webkit-app-region: no-drag;
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
    position: relative;
    cursor: pointer;
  }

  .copied-toast {
    position: absolute;
    top: -20px;
    right: 10px;
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
    padding: 2px 8px;
    border-radius: 5px;
    font-size: 12px;
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
    line-height: 1.5;
    font-size: var(--font-size-sm);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .attachment-block {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    background: var(--bg-tertiary);
    padding: var(--spacing-sm) var(--spacing-md);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-primary);
  }

  .message.user .attachment-block {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .attachment-icon {
    flex-shrink: 0;
    width: 20px;
    height: 20px;
  }

  .attachment-name {
    font-weight: var(--font-weight-medium);
    font-size: var(--font-size-xs);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
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

  .filename-pill {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    background: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-full);
    padding: 4px var(--spacing-sm);
    font-size: var(--font-size-xs);
    white-space: nowrap;
    overflow: hidden;
    flex-shrink: 1;
  }

  .pill-text {
    color: var(--text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 120px;
  }

  .remove-pill-button {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 0;
    margin: 0;
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-full);
    flex-shrink: 0;
  }

  .remove-pill-button:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .remove-pill-button > :global(svg) {
    width: 12px;
    height: 12px;
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

  .attachment-button {
    width: 40px;
    height: 40px;
    border-radius: var(--radius-full);
    background: transparent;
    border: none;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: var(--transition-normal);
    flex-shrink: 0;
  }

  .attachment-button:hover {
    background: var(--bg-secondary);
    transform: scale(1.05);
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