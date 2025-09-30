<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { _ } from 'svelte-i18n';

  let settings = $state({
    api_key: '',
    api_url: '',
    model_name: '',
    shortcut: '',
    system_prompt: '',
    api_type: 'openai'
  });
  let message = $state('');
  let openSection = $state('aiConfig'); // aiConfig, appSettings
  let isRecording = $state(false);
  let previousShortcut = '';

  const handleShortcutKeydown = (event: KeyboardEvent) => {
    event.preventDefault();
    event.stopPropagation();

    if (event.key === 'Escape') {
      cancelRecording();
      return;
    }

    if (['Control', 'Alt', 'Shift', 'Meta'].includes(event.key)) {
        return;
    }

    const parts = [];
    if (event.ctrlKey) parts.push('Ctrl');
    if (event.altKey) parts.push('Alt');
    if (event.shiftKey) parts.push('Shift');
    if (event.metaKey) parts.push('Super');

    const key = event.key.toUpperCase();
    let finalKey = key;
    if (key.startsWith('ARROW')) {
        finalKey = key.substring(5);
    }
    if (finalKey === ' ') {
        finalKey = 'SPACE';
    }
    parts.push(finalKey);

    const isChar = event.key.length === 1 && event.key.match(/[a-zA-Z0-9]/);
    if (parts.length > 0 && (!isChar || parts.length > 1)) {
        settings.shortcut = parts.join('+');
    } else {
        settings.shortcut = previousShortcut;
    }

    isRecording = false;
    window.removeEventListener('keydown', handleShortcutKeydown, { capture: true });
  };

  function startRecording() {
    previousShortcut = settings.shortcut;
    isRecording = true;
    settings.shortcut = $_('settings.appSettings.shortcutRecording');
    window.addEventListener('keydown', handleShortcutKeydown, { capture: true });
  }

  function cancelRecording() {
    isRecording = false;
    settings.shortcut = previousShortcut;
    window.removeEventListener('keydown', handleShortcutKeydown, { capture: true });
  }

  onMount(() => {
    invoke('get_settings').then((loadedSettings) => {
      settings = { ...settings, ...loadedSettings as typeof settings };
      if (settings.api_url && !settings.api_url.includes('/chat/completions')) {
        settings.api_url = normalizeApiUrl(settings.api_url);
      }
    }).catch((e) => {
      message = $_('settings.messages.loadError', { values: { error: e }});
    });

    return () => {
        window.removeEventListener('keydown', handleShortcutKeydown, { capture: true });
    };
  });

  async function handleSave() {
    try {
      const processedSettings = { ...settings };
      processedSettings.api_url = normalizeApiUrl(processedSettings.api_url);
      
      await invoke('set_settings', { settings: processedSettings });
      settings.api_url = processedSettings.api_url;
      message = $_('settings.messages.saveSuccess');
      setTimeout(() => { message = '' }, 3000);
    } catch (e) {
      message = $_('settings.messages.saveError', { values: { error: e }});
    }
  }

  function handleUrlBlur() {
    if (settings.api_url && !settings.api_url.includes('/chat/completions')) {
      settings.api_url = normalizeApiUrl(settings.api_url);
    }
  }

  function normalizeApiUrl(url: string): string {
    if (!url) return url;
    if (url.includes('/chat/completions')) return url;
    let normalizedUrl = url.replace(/\/+$/, '');
    if (normalizedUrl.endsWith('/v1')) return normalizedUrl + '/chat/completions';
    if (normalizedUrl.endsWith('/api')) return normalizedUrl + '/v1/chat/completions';
    if (normalizedUrl.endsWith('/chat')) return normalizedUrl + '/completions';
    if (normalizedUrl.includes('api.deepseek.com')) return normalizedUrl + '/v1/chat/completions';
    return normalizedUrl + '/v1/chat/completions';
  }

  function toggleSection(section: string) {
      if (openSection === section) {
          openSection = '';
      } else {
          openSection = section;
      }
  }
</script>

<main class="glass">
  <div class="settings-container">
    <div class="settings-header">
      <a href="/" class="back-button">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M19 12H5"></path>
          <path d="M12 19l-7-7 7-7"></path>
        </svg>
        {$_('settings.back')}
      </a>
      <h2>{$_('settings.title')}</h2>
    </div>
    
    <div class="settings-content">
      <div class="settings-section">
        <h3 class="accordion-header" onclick={() => toggleSection('aiConfig')}>
          {$_('settings.aiConfig.title')}
          <span class="chevron {openSection === 'aiConfig' ? 'open' : ''}"></span>
        </h3>
        {#if openSection === 'aiConfig'}
        <div class="accordion-content">
            <div class="form-group">
              <label for="system-prompt">{$_('settings.aiConfig.systemPrompt')}</label>
              <textarea id="system-prompt" bind:value={settings.system_prompt} rows="3" placeholder={$_('settings.aiConfig.systemPromptPlaceholder')}></textarea>
            </div>

            <div class="form-group">
              <label for="api-type">{$_('settings.aiConfig.apiType')}</label>
              <select id="api-type" bind:value={settings.api_type}>
                <option value="openai">{$_('settings.aiConfig.openai')}</option>
                <option value="openai-compatible">{$_('settings.aiConfig.openaiCompatible')}</option>
              </select>
            </div>

            <div class="form-group">
              <label for="api-key">{$_('settings.aiConfig.apiKey')}</label>
              <input id="api-key" type="password" bind:value={settings.api_key} placeholder={$_('settings.aiConfig.apiKeyPlaceholder')} />
            </div>

            <div class="form-group">
              <label for="api-url">{$_('settings.aiConfig.apiEndpoint')}</label>
              <input 
                id="api-url" 
                type="text" 
                bind:value={settings.api_url} 
                placeholder={$_('settings.aiConfig.apiEndpointPlaceholder')} 
                onblur={handleUrlBlur}
              />
              <p class="hint">
                {#if settings.api_url && !settings.api_url.includes('/chat/completions')}
                  {$_('settings.aiConfig.apiEndpointHintSave', { values: { url: normalizeApiUrl(settings.api_url) } })}
                {:else}
                  {$_('settings.aiConfig.apiEndpointHint')}
                {/if}
              </p>
            </div>

            <div class="form-group">
              <label for="model-name">{$_('settings.aiConfig.modelName')}</label>
              <input id="model-name" type="text" bind:value={settings.model_name} placeholder={$_('settings.aiConfig.modelNamePlaceholder')} />
            </div>
        </div>
        {/if}
      </div>

      <div class="settings-section">
        <h3 class="accordion-header" onclick={() => toggleSection('appSettings')}>
            {$_('settings.appSettings.title')}
            <span class="chevron {openSection === 'appSettings' ? 'open' : ''}"></span>
        </h3>
        {#if openSection === 'appSettings'}
        <div class="accordion-content">
            <div class="form-group">
              <label for="shortcut">{$_('settings.appSettings.shortcut')}</label>
              <div class="shortcut-recorder">
                <input id="shortcut" type="text" readonly bind:value={settings.shortcut} placeholder={$_('settings.appSettings.shortcutPlaceholder')} />
                {#if isRecording}
                    <button class="secondary-button" onclick={cancelRecording}>{$_('settings.appSettings.shortcutCancel')}</button>
                {:else}
                    <button class="primary-button" onclick={startRecording}>{$_('settings.appSettings.shortcutRecord')}</button>
                {/if}
              </div>
              {#if isRecording}
                <p class="hint">{$_('settings.appSettings.shortcutHintRecording')}</p>
              {:else}
                <p class="hint">{$_('settings.appSettings.shortcutHint')}</p>
              {/if}
            </div>
        </div>
        {/if}
      </div>

      <div class="actions">
        <button onclick={handleSave} class="primary-button">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
            <polyline points="17 21 17 13 7 13 7 21"></polyline>
            <polyline points="7 3 7 8 15 8"></polyline>
          </svg>
          {$_('settings.actions.save')}
        </button>
        <button class="secondary-button" onclick={() => invoke('open_config_directory')}>
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7"></path>
            <line x1="16" y1="5" x2="22" y2="5"></line>
            <line x1="19" y1="2" x2="19" y2="8"></line>
          </svg>
          {$_('settings.actions.showConfig')}
        </button>
      </div>

      {#if message}
        <div class="message-banner {message.includes('Error') || message.includes('错') ? 'error' : 'success'}">
          {message}
        </div>
      {/if}
    </div>
  </div>
</main>

<style>
  main {
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    font-family: var(--font-family);
    background: var(--bg-primary);
  }

  .settings-container {
    padding: var(--spacing-md);
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    box-sizing: border-box;
    max-width: 480px;
    margin: 0 auto;
    gap: var(--spacing-sm);
  }

  .settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--spacing-md);
    position: relative;
  }

  .back-button {
    display: inline-flex;
    align-items: center;
    gap: var(--spacing-sm);
    background: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    color: var(--text-primary);
    padding: var(--spacing-sm) var(--spacing-md);
    border-radius: var(--radius-md);
    text-decoration: none;
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    transition: var(--transition-normal);
    position: absolute;
    left: 0;
    top: 0;
  }

  .back-button:hover {
    background: var(--bg-tertiary);
    transform: translateX(-2px);
  }

  h2 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    margin: 0;
    color: var(--text-primary);
    text-align: center;
    flex-grow: 1;
  }

  .settings-content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .settings-section {
    background: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-soft);
    transition: all var(--transition-fast);
  }
  .settings-section.flat {
      padding: var(--spacing-lg);
  }

  .accordion-header {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
    padding: var(--spacing-md) var(--spacing-lg);
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .accordion-content {
      padding: 0 var(--spacing-lg) var(--spacing-lg) var(--spacing-lg);
      animation: fadeIn 0.2s ease-out;
  }

  .static-header {
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
    margin-bottom: var(--spacing-md);
  }

  .chevron {
    width: 1em;
    height: 1em;
    background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e");
    background-repeat: no-repeat;
    background-position: center;
    transition: transform var(--transition-fast);
  }

  .chevron.open {
      transform: rotate(180deg);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
    margin-bottom: var(--spacing-md);
  }
  .form-group:last-child {
      margin-bottom: 0;
  }

  label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--text-primary);
  }

  input, textarea, select {
    border-radius: var(--radius-md);
    border: 1px solid var(--border-primary);
    padding: var(--spacing-sm) var(--spacing-md);
    font-size: var(--font-size-sm);
    font-family: inherit;
    color: var(--text-primary);
    background: var(--bg-primary);
    transition: var(--transition-normal);
    outline: none;
    resize: vertical;
    width: 100%;
  }

  input:focus, textarea:focus, select:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 2px var(--shadow-glow);
  }

  input::placeholder, textarea::placeholder {
    color: var(--text-muted);
  }

  select {
    appearance: none;
    background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e");
    background-position: right 0.5rem center;
    background-repeat: no-repeat;
    background-size: 1.5em 1.5em;
    padding-right: 2.5rem;
  }

  .hint {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    margin: var(--spacing-xs) 0 0 0;
    padding: 0;
  }

  .actions {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
    margin-top: var(--spacing-md);
  }

  .primary-button, .secondary-button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    font-family: inherit;
    cursor: pointer;
    transition: var(--transition-normal);
    border: none;
    outline: none;
  }

  .primary-button {
    background: var(--primary);
    color: var(--bg-primary);
  }

  .primary-button:hover {
    transform: translateY(-1px);
    box-shadow: var(--shadow-soft);
    background: var(--primary-hover);
  }

  .secondary-button {
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-primary);
  }

  .secondary-button:hover {
    background: var(--bg-tertiary);
    border-color: var(--primary);
  }

  .message-banner {
    padding: var(--spacing-md) var(--spacing-lg);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    text-align: center;
    animation: slideIn 0.3s ease-out;
  }

  .message-banner.success {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
    border: 1px solid rgba(34, 197, 94, 0.3);
  }

  .message-banner.error {
    background: rgba(239, 68, 68, 0.2);
    color: #ef4444;
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  .shortcut-recorder {
      display: flex;
      gap: var(--spacing-sm);
  }

  .shortcut-recorder input {
      flex-grow: 1;
      background-color: var(--bg-tertiary) !important;
  }

  .shortcut-recorder button {
      flex-shrink: 0;
  }

  /* Responsive design */
  @media (max-width: 640px) {
    .settings-container {
      padding: var(--spacing-md);
    }
    
    .settings-section {
      padding: var(--spacing-md);
    }
    
    .actions {
      flex-direction: column;
    }
  }
</style>
