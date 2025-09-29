<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { theme } from '$lib/stores/theme';
  import { getActualTheme } from '$lib/stores/theme';

  let settings = $state({
    api_key: '',
    api_url: '',
    model_name: '',
    shortcut: '',
    system_prompt: '',
    api_type: 'openai'
  });
  let message = $state('');
  let currentTheme: 'light' | 'dark' | 'auto' = $state('auto');
  let actualTheme: 'light' | 'dark' = $state('light');

  onMount(() => {
    // Load settings
    invoke('get_settings').then((loadedSettings) => {
      settings = { ...settings, ...loadedSettings as typeof settings };
      
      // 确保加载的设置 URL 格式正确
      if (settings.api_url && !settings.api_url.includes('/chat/completions')) {
        settings.api_url = normalizeApiUrl(settings.api_url);
      }
    }).catch((e) => {
      message = `Error getting settings: ${e}`;
    });

    // Initialize theme
    const unsubscribe = theme.subscribe(value => {
      currentTheme = value;
      actualTheme = getActualTheme(value);
    });

    return unsubscribe;
  });

  async function handleSave() {
    try {
      // 确保 URL 格式正确
      const processedSettings = { ...settings };
      processedSettings.api_url = normalizeApiUrl(processedSettings.api_url);
      
      await invoke('set_settings', { settings: processedSettings });
      settings.api_url = processedSettings.api_url; // 更新显示的值
      message = 'Settings saved successfully!';
      setTimeout(() => { message = '' }, 3000);
    } catch (e) {
      message = `Error saving settings: ${e}`;
    }
  }

  function handleUrlBlur() {
    // 当输入框失去焦点时自动补全 URL
    if (settings.api_url && !settings.api_url.includes('/chat/completions')) {
      settings.api_url = normalizeApiUrl(settings.api_url);
    }
  }

  function normalizeApiUrl(url: string): string {
    if (!url) return url;
    
    // 如果已经包含 /chat/completions，直接返回
    if (url.includes('/chat/completions')) {
      return url;
    }
    
    // 移除末尾的斜杠
    let normalizedUrl = url.replace(/\/+$/, '');
    
    // 检查常见的 API 端点模式
    if (normalizedUrl.endsWith('/v1')) {
      return normalizedUrl + '/chat/completions';
    }
    
    if (normalizedUrl.endsWith('/api')) {
      return normalizedUrl + '/v1/chat/completions';
    }
    
    if (normalizedUrl.endsWith('/chat')) {
      return normalizedUrl + '/completions';
    }
    
    // 检查是否已经是完整的 OpenAI 兼容端点（如 DeepSeek）
    if (normalizedUrl.includes('api.deepseek.com')) {
      return normalizedUrl + '/v1/chat/completions';
    }
    
    // 默认添加 /v1/chat/completions
    return normalizedUrl + '/v1/chat/completions';
  }

  function setTheme(newTheme: 'light' | 'dark' | 'auto') {
    theme.set(newTheme);
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
        Back
      </a>
      <h2>Settings</h2>
    </div>
    
    <div class="settings-content">
      <div class="settings-section">
        <h3>AI Configuration</h3>
        <div class="form-group">
          <label for="system-prompt">System Prompt</label>
          <textarea id="system-prompt" bind:value={settings.system_prompt} rows="4" placeholder="Define the AI's behavior and personality..."></textarea>
        </div>

        <div class="form-group">
          <label for="api-type">API Type</label>
          <select id="api-type" bind:value={settings.api_type}>
            <option value="openai">OpenAI</option>
            <option value="openai-compatible">OpenAI Compatible</option>
          </select>
        </div>

        <div class="form-group">
          <label for="api-key">API Key</label>
          <input id="api-key" type="password" bind:value={settings.api_key} placeholder="Enter your API key" />
        </div>

        <div class="form-group">
          <label for="api-url">API Endpoint URL</label>
          <input 
            id="api-url" 
            type="text" 
            bind:value={settings.api_url} 
            placeholder="e.g., https://api.openai.com/v1/chat/completions" 
            onblur={handleUrlBlur}
          />
          <p class="hint">
            {#if settings.api_url && !settings.api_url.includes('/chat/completions')}
              Will be saved as: {normalizeApiUrl(settings.api_url)}
            {:else}
              We'll automatically append /chat/completions if needed
            {/if}
          </p>
        </div>

        <div class="form-group">
          <label for="model-name">Model Name</label>
          <input id="model-name" type="text" bind:value={settings.model_name} placeholder="e.g., gpt-4o-mini or deepseek-chat" />
        </div>
      </div>

      <div class="settings-section">
        <h3>Application Settings</h3>
        <div class="form-group">
          <label for="shortcut">Global Shortcut</label>
          <input id="shortcut" type="text" bind:value={settings.shortcut} placeholder="e.g., Alt+Space" />
          <p class="hint">Use modifiers like Ctrl, Alt, Shift, Super. Separate with "+".</p>
        </div>
      </div>

      <div class="settings-section">
        <h3>Appearance</h3>
        <div class="theme-options">
          <button class="theme-option {currentTheme === 'light' ? 'active' : ''}" onclick={() => setTheme('light')} aria-pressed={currentTheme === 'light'}>
            <div class="theme-preview light">
              <div class="preview-header"></div>
              <div class="preview-content">
                <div class="preview-message user"></div>
                <div class="preview-message assistant"></div>
              </div>
              <div class="preview-input"></div>
            </div>
            <span class="theme-label">Light</span>
          </button>
          
          <button class="theme-option {currentTheme === 'dark' ? 'active' : ''}" onclick={() => setTheme('dark')} aria-pressed={currentTheme === 'dark'}>
            <div class="theme-preview dark">
              <div class="preview-header"></div>
              <div class="preview-content">
                <div class="preview-message user"></div>
                <div class="preview-message assistant"></div>
              </div>
              <div class="preview-input"></div>
            </div>
            <span class="theme-label">Dark</span>
          </button>
          
          <button class="theme-option {currentTheme === 'auto' ? 'active' : ''}" onclick={() => setTheme('auto')} aria-pressed={currentTheme === 'auto'}>
            <div class="theme-preview auto">
              <div class="preview-header"></div>
              <div class="preview-content">
                <div class="preview-message user"></div>
                <div class="preview-message assistant"></div>
              </div>
              <div class="preview-input"></div>
            </div>
            <span class="theme-label">Auto</span>
            <span class="theme-description">Follow system</span>
          </button>
        </div>
        
        <div class="current-theme-info">
          <p>Current theme: <strong>{actualTheme}</strong> {currentTheme === 'auto' ? '(system preference)' : ''}</p>
        </div>
      </div>

      <div class="actions">
        <button onclick={handleSave} class="primary-button">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
            <polyline points="17 21 17 13 7 13 7 21"></polyline>
            <polyline points="7 3 7 8 15 8"></polyline>
          </svg>
          Save Settings
        </button>
        <button class="secondary-button" onclick={() => invoke('open_config_directory')}>
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7"></path>
            <line x1="16" y1="5" x2="22" y2="5"></line>
            <line x1="19" y1="2" x2="19" y2="8"></line>
          </svg>
          Show Config Directory
        </button>
      </div>

      {#if message}
        <div class="message-banner {message.includes('Error') ? 'error' : 'success'}">
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
    padding: var(--spacing-lg);
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    box-sizing: border-box;
    max-width: 500px;
    margin: 0 auto;
    gap: var(--spacing-lg);
  }

  .settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--spacing-2xl);
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
    font-size: var(--font-size-3xl);
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
    gap: var(--spacing-2xl);
  }

  .settings-section {
    background: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-lg);
    padding: var(--spacing-lg);
    box-shadow: var(--shadow-soft);
  }

  h3 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
    margin-bottom: var(--spacing-lg);
    border-bottom: 1px solid var(--border-secondary);
    padding-bottom: var(--spacing-sm);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    margin-bottom: var(--spacing-lg);
  }

  label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--text-primary);
  }

  input, textarea, select {
    border-radius: var(--radius-md);
    border: 1px solid var(--border-primary);
    padding: var(--spacing-md) var(--spacing-lg);
    font-size: var(--font-size-base);
    font-family: inherit;
    color: var(--text-primary);
    background: var(--bg-primary);
    transition: var(--transition-normal);
    outline: none;
    resize: vertical;
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
  }

  .primary-button, .secondary-button {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-md) var(--spacing-lg);
    border-radius: var(--radius-md);
    font-size: var(--font-size-base);
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
    transform: translateY(-2px);
    box-shadow: 0 8px 25px var(--shadow-glow);
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

  /* Theme Options */
  .theme-options {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: var(--spacing-md);
    margin-bottom: var(--spacing-lg);
  }

  .theme-option {
    cursor: pointer;
    border: 2px solid var(--border-primary);
    border-radius: var(--radius-md);
    padding: var(--spacing-md);
    transition: var(--transition-normal);
    background: var(--bg-primary);
    text-align: center;
  }

  .theme-option:hover {
    border-color: var(--primary);
    transform: translateY(-2px);
    box-shadow: var(--shadow-soft);
  }

  .theme-option.active {
    border-color: var(--primary);
    background: var(--bg-tertiary);
  }

  .theme-preview {
    width: 100%;
    height: 80px;
    border-radius: var(--radius-sm);
    overflow: hidden;
    margin-bottom: var(--spacing-sm);
    border: 1px solid var(--border-primary);
  }

  .theme-preview.light {
    background: #ffffff;
  }

  .theme-preview.dark {
    background: #0f172a;
  }

  .theme-preview.auto {
    background: linear-gradient(135deg, #ffffff 50%, #0f172a 50%);
  }

  .preview-header {
    height: 12px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-primary);
  }

  .preview-content {
    padding: 6px;
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .preview-message {
    height: 6px;
    border-radius: 3px;
  }

  .preview-message.user {
    background: var(--blue-primary);
    width: 60%;
    margin-left: auto;
  }

  .preview-message.assistant {
    background: var(--bg-tertiary);
    width: 80%;
  }

  .preview-input {
    height: 8px;
    background: var(--bg-primary);
    border-top: 1px solid var(--border-primary);
    margin-top: auto;
  }

  .theme-label {
    display: block;
    font-weight: var(--font-weight-medium);
    color: var(--text-primary);
    font-size: var(--font-size-sm);
  }

  .theme-description {
    display: block;
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    margin-top: var(--spacing-xs);
  }

  .current-theme-info {
    padding: var(--spacing-md);
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-primary);
  }

  .current-theme-info p {
    margin: 0;
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
  }

  /* Responsive design */
  @media (max-width: 640px) {
    .settings-container {
      padding: var(--spacing-lg);
    }
    
    .settings-section {
      padding: var(--spacing-lg);
    }
    
    .actions {
      flex-direction: column;
    }
    
    .theme-options {
      grid-template-columns: 1fr;
    }
  }
</style>
