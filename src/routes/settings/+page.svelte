<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { _ } from 'svelte-i18n';
  import { clearChatShortcut, borderless, borderlessShortcut } from '$lib/stores/settings.store';

  let settings = $state({
    api_key: '',
    api_url: '',
    model_name: '',
    shortcut: '',
    system_prompt: '',
    api_type: 'openai',
    system_prompt_preset: 'default'
  });
  let message = $state('');
  let openSection = $state('aiConfig'); // aiConfig, appSettings
  let isRecording = $state(false);
  let isRecordingClearChat = $state(false);
  let isRecordingBorderless = $state(false);
  let previousShortcut = '';
  let previousClearChatShortcut = '';
  let previousBorderlessShortcut = '';

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

  const handleClearChatShortcutKeydown = (event: KeyboardEvent) => {
    event.preventDefault();
    event.stopPropagation();

    if (event.key === 'Escape') {
      cancelClearChatRecording();
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
      clearChatShortcut.set(parts.join('+'));
    } else {
      clearChatShortcut.set(previousClearChatShortcut);
    }

    isRecordingClearChat = false;
    window.removeEventListener('keydown', handleClearChatShortcutKeydown, { capture: true });
  };

  function startClearChatRecording() {
    previousClearChatShortcut = $clearChatShortcut;
    isRecordingClearChat = true;
    clearChatShortcut.set($_('settings.appSettings.shortcutRecording'));
    window.addEventListener('keydown', handleClearChatShortcutKeydown, { capture: true });
  }

  function cancelClearChatRecording() {
    isRecordingClearChat = false;
    clearChatShortcut.set(previousClearChatShortcut);
    window.removeEventListener('keydown', handleClearChatShortcutKeydown, { capture: true });
  }

  const handleBorderlessShortcutKeydown = (event: KeyboardEvent) => {
    event.preventDefault();
    event.stopPropagation();

    if (event.key === 'Escape') {
      cancelBorderlessRecording();
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
      borderlessShortcut.set(parts.join('+'));
    } else {
      borderlessShortcut.set(previousBorderlessShortcut);
    }

    isRecordingBorderless = false;
    window.removeEventListener('keydown', handleBorderlessShortcutKeydown, { capture: true });
  };

  function startBorderlessRecording() {
    previousBorderlessShortcut = $borderlessShortcut;
    isRecordingBorderless = true;
    borderlessShortcut.set($_('settings.appSettings.shortcutRecording'));
    window.addEventListener('keydown', handleBorderlessShortcutKeydown, { capture: true });
  }

  function cancelBorderlessRecording() {
    isRecordingBorderless = false;
    borderlessShortcut.set(previousBorderlessShortcut);
    window.removeEventListener('keydown', handleBorderlessShortcutKeydown, { capture: true });
  }

  



  onMount(() => {
    invoke('get_settings').then((loadedSettings: any) => {
      const { clear_chat_shortcut, ...rest } = loadedSettings;
      settings = { ...settings, ...rest };
      
      // Set system_prompt_preset based on current system_prompt
      if (settings.system_prompt === "You are a helpful assistant.") {
        settings.system_prompt_preset = 'default';
      } else if (settings.system_prompt === "Your function is to distill every query to its absolute essence. Provide the single most critical piece of information as a declarative statement. Maximum signal, zero noise. Your response should rarely exceed one sentence.") {
        settings.system_prompt_preset = 'minimal';
      } else {
        settings.system_prompt_preset = 'custom';
      }
      
      // 处理API密钥显示：如果已配置且不是默认值，显示占位符点
      if (settings.api_key && settings.api_key !== 'your_api_key_here') {
        actualApiKey = settings.api_key;
        settings.api_key = '••••••••••••••••••••';
      } else {
        actualApiKey = settings.api_key;
      }
      
      if (clear_chat_shortcut != null) {
        clearChatShortcut.set(clear_chat_shortcut);
      }
      if (settings.api_url && !settings.api_url.includes('/chat/completions')) {
        settings.api_url = normalizeApiUrl(settings.api_url);
      }
    }).catch((e) => {
      message = $_('settings.messages.loadError', { values: { error: String(e) }});
    });

    return () => {
        window.removeEventListener('keydown', handleShortcutKeydown, { capture: true });
        window.removeEventListener('keydown', handleClearChatShortcutKeydown, { capture: true });
        window.removeEventListener('keydown', handleBorderlessShortcutKeydown, { capture: true });
    };
  });

  async function handleSave(event: Event) {
    event.preventDefault();
    try {
      const processedSettings = { 
        ...settings, 
        clear_chat_shortcut: $clearChatShortcut,
        borderless_shortcut: $borderlessShortcut 
      };
      processedSettings.api_url = normalizeApiUrl(processedSettings.api_url);
      
      // 处理API密钥：如果显示的是占位符点，使用实际值；如果用户清空了，则清空
      if (processedSettings.api_key === '••••••••••••••••••••') {
        // 用户没有修改API密钥，保持原来的值
        processedSettings.api_key = actualApiKey;
      } else if (processedSettings.api_key === '') {
        // 用户清空了API密钥，设置为空
        actualApiKey = '';
      } else {
        // 用户输入了新值，更新实际值
        actualApiKey = processedSettings.api_key;
      }
      
      await invoke('set_settings', { settings: processedSettings });
      settings.api_url = processedSettings.api_url;
      
      // 保存后显示占位符点（如果API密钥存在且不是默认值）
      if (actualApiKey && actualApiKey !== 'your_api_key_here') {
        settings.api_key = '••••••••••••••••••••';
      }
      
      message = $_('settings.messages.saveSuccess');
      setTimeout(() => { message = '' }, 3000);
    } catch (e) {
      message = $_('settings.messages.saveError', { values: { error: String(e) }});
    }
  }

  function handleUrlBlur() {
    if (settings.api_url && !settings.api_url.includes('/chat/completions')) {
      settings.api_url = normalizeApiUrl(settings.api_url);
    }
  }

  function normalizeApiUrl(url: string): string {
    if (!url) return url;
    
    // 移除可能的引号和多余空格
    let normalizedUrl = url.trim().replace(/^"|"$/g, '');
    
    // 如果已经包含完整的路径，直接返回
    if (normalizedUrl.includes('/chat/completions')) {
      return normalizedUrl;
    }
    
    // 移除末尾的斜杠
    normalizedUrl = normalizedUrl.replace(/\/+$/, '');
    
    // 根据不同的端点格式添加适当的路径
    if (normalizedUrl.endsWith('/v1')) {
      return normalizedUrl + '/chat/completions';
    }
    if (normalizedUrl.endsWith('/api')) {
      return normalizedUrl + '/v1/chat/completions';
    }
    if (normalizedUrl.endsWith('/chat')) {
      return normalizedUrl + '/completions';
    }
    
    // 对于常见的 API 提供商，添加标准路径
    if (normalizedUrl.includes('api.deepseek.com')) {
      return normalizedUrl + '/v1/chat/completions';
    }
    
    // 对于其他自定义端点，假设它们需要完整的路径
    return normalizedUrl + '/v1/chat/completions';
  }

  function toggleSection(section: string) {
      if (openSection === section) {
          openSection = '';
      } else {
          openSection = section;
      }
  }

  function handlePresetChange() {
    const preset = settings.system_prompt_preset;
    if (preset === 'default') {
      settings.system_prompt = "You are a helpful assistant.";
    } else if (preset === 'minimal') {
      settings.system_prompt = "Your function is to distill every query to its absolute essence. Provide the single most critical piece of information as a declarative statement. Maximum signal, zero noise. Your response should rarely exceed one sentence.";
    }
    // For 'custom', don't change the system_prompt value
  }

  let apiKeyDisplayValue = $state('');
  let actualApiKey = $state('');

  function handleApiKeyFocus() {
    // 当获得焦点时，显示实际值
    settings.api_key = actualApiKey;
  }

  function handleApiKeyBlur() {
    // 当失去焦点时，如果API密钥已配置且不是默认值，显示占位符点
    if (settings.api_key && settings.api_key !== 'your_api_key_here') {
      actualApiKey = settings.api_key;
      settings.api_key = '••••••••••••••••••••';
    } else if (settings.api_key === '') {
      // 用户清空了输入框，更新实际值
      actualApiKey = '';
    }
  }
</script>

<main class="glass" data-tauri-drag-region>
  <div class="settings-container">
    <div class="settings-header" data-tauri-drag-region>
      <a href="/" class="back-button">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M19 12H5"></path>
          <path d="M12 19l-7-7 7-7"></path>
        </svg>
        {$_('settings.back')}
      </a>
      <h2 data-tauri-drag-region>{$_('settings.title')}</h2>
    </div>
    
    <form onsubmit={handleSave}>
      <div class="settings-content">
        <div class="settings-section">
          <button type="button" class="accordion-header" onclick={() => toggleSection('aiConfig')} aria-expanded={openSection === 'aiConfig'}>
            <span class="accordion-title">{$_('settings.aiConfig.title')}</span>
            <span class="chevron {openSection === 'aiConfig' ? 'open' : ''}"></span>
          </button>
          {#if openSection === 'aiConfig'}
          <div class="accordion-content form-grid">
              <div class="form-group">
                <div class="form-group-header">
                  <label for="system-prompt-preset">{$_('settings.aiConfig.systemPromptPreset')}</label>
                </div>
                <select id="system-prompt-preset" bind:value={settings.system_prompt_preset} onchange={handlePresetChange}>
                  <option value="default">{$_('settings.aiConfig.systemPromptPresetDefault')} - {$_('settings.aiConfig.systemPromptPresetDefaultDesc')}</option>
                  <option value="minimal">{$_('settings.aiConfig.systemPromptPresetMinimal')} - {$_('settings.aiConfig.systemPromptPresetMinimalDesc')}</option>
                  <option value="custom">{$_('settings.aiConfig.systemPromptPresetCustom')} - {$_('settings.aiConfig.systemPromptPresetCustomDesc')}</option>
                </select>
              </div>

              <div class="form-group span-2">
                <div class="form-group-header">
                  <label for="system-prompt">{$_('settings.aiConfig.systemPrompt')}</label>
                </div>
                <textarea id="system-prompt" bind:value={settings.system_prompt} rows="3" placeholder={$_('settings.aiConfig.systemPromptPlaceholder')}></textarea>
              </div>

              <div class="form-group">
                <div class="form-group-header">
                  <label for="api-type">{$_('settings.aiConfig.apiType')}</label>
                </div>
                <select id="api-type" bind:value={settings.api_type}>
                  <option value="openai">{$_('settings.aiConfig.openai')}</option>
                  <option value="openai-compatible">{$_('settings.aiConfig.openaiCompatible')}</option>
                </select>
              </div>

              <div class="form-group">
                <div class="form-group-header">
                  <label for="api-key">{$_('settings.aiConfig.apiKey')}</label>
                </div>
                <input type="text" autocomplete="username" style="display:none;">
                <input 
                  id="api-key" 
                  type="password" 
                  bind:value={settings.api_key} 
                  placeholder={$_('settings.aiConfig.apiKeyPlaceholder')}
                  onfocus={handleApiKeyFocus}
                  onblur={handleApiKeyBlur}
                  autocomplete="current-password"
                />
              </div>

              <div class="form-group span-2">
                <div class="form-group-header">
                  <label for="api-url">{$_('settings.aiConfig.apiEndpoint')}</label>
                </div>
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
                <div class="form-group-header">
                  <label for="model-name">{$_('settings.aiConfig.modelName')}</label>
                </div>
                <input id="model-name" type="text" bind:value={settings.model_name} placeholder={$_('settings.aiConfig.modelNamePlaceholder')} />
              </div>


          </div>
          {/if}
        </div>

        <div class="settings-section">
          <button type="button" class="accordion-header" onclick={() => toggleSection('appSettings')} aria-expanded={openSection === 'appSettings'}>
            <span class="accordion-title">{$_('settings.appSettings.title')}</span>
            <span class="chevron {openSection === 'appSettings' ? 'open' : ''}"></span>
          </button>
          {#if openSection === 'appSettings'}
          <div class="accordion-content form-grid">
              <div class="form-group span-2">
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

              <div class="form-group span-2">
                <label for="clear-chat-shortcut">{$_('settings.appSettings.clearChatShortcut')}</label>
                <div class="shortcut-recorder">
                  <input id="clear-chat-shortcut" type="text" readonly bind:value={$clearChatShortcut} />
                  {#if isRecordingClearChat}
                      <button type="button" class="secondary-button" onclick={cancelClearChatRecording}>{$_('settings.appSettings.shortcutCancel')}</button>
                  {:else}
                      <button type="button" class="primary-button" onclick={startClearChatRecording}>{$_('settings.appSettings.shortcutRecord')}</button>
                  {/if}
                </div>
                {#if isRecordingClearChat}
                  <p class="hint">{$_('settings.appSettings.shortcutHintRecording')}</p>
                {/if}
                {#if isRecordingClearChat}
                  <p class="hint">{$_('settings.appSettings.clearChatShortcutHint')}</p>
                {/if}
              </div>

              <div class="form-group span-2">
                <label for="borderless-shortcut">{$_('settings.appSettings.borderless')}</label>
                <div class="shortcut-recorder">
                  <input id="borderless-shortcut" type="text" readonly bind:value={$borderlessShortcut} />
                  {#if isRecordingBorderless}
                    <button type="button" class="secondary-button" onclick={cancelBorderlessRecording}>{$_('settings.appSettings.shortcutCancel')}</button>
                  {:else}
                    <button type="button" class="primary-button" onclick={startBorderlessRecording}>{$_('settings.appSettings.shortcutRecord')}</button>
                  {/if}
                </div>
                {#if isRecordingBorderless}
                  <p class="hint">{$_('settings.appSettings.shortcutHintRecording')}</p>
                {:else}
                  <p class="hint">{$_('settings.appSettings.borderlessShortcutHint')}</p>
                {/if}
              </div>

              
          </div>
          {/if}
        </div>

        <div class="actions">
          <button type="submit" class="primary-button">
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

        <div class="settings-section flat about-section">
          <h3 class="static-header">{$_('settings.about.title')}</h3>
          <div class="about-content">
            <p><strong>{$_('settings.about.author')}:</strong> YHZZ9457</p>
            <p><strong>{$_('settings.about.repository')}:</strong> <a href="https://github.com/YHZZ9457/AI-window" target="_blank">https://github.com/YHZZ9457/AI-window</a></p>
            <p><strong>{$_('settings.about.contact')}:</strong> a15952149553@gmail.com / QQ: 2680159691</p>
          </div>
        </div>
      </div>
    </form>
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
    padding: var(--spacing-xl);
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    box-sizing: border-box;
    max-width: 560px;
    margin: 0 auto;
    gap: var(--spacing-lg);
  }

  .settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--spacing-md);
    position: relative;
    -webkit-app-region: drag;
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
    -webkit-app-region: no-drag;
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
    gap: var(--spacing-md);
  }

  .settings-section {
    background: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-soft);
    transition: all var(--transition-fast);
    overflow: hidden;
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
    background: transparent;
    border: none;
    width: 100%;
    text-align: left;
    transition: all var(--transition-fast);
  }

  .accordion-header:hover {
    background: var(--bg-tertiary);
  }

  .accordion-content {
      padding: 0 var(--spacing-lg) var(--spacing-lg) var(--spacing-lg);
      animation: slideDown 0.3s ease-out;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
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
    gap: var(--spacing-sm);
  }

  

  .form-group-header {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--spacing-xl);
  }

  .form-group.span-2 {
    grid-column: span 2;
  }

  @media (max-width: 640px) {
    .form-grid {
      grid-template-columns: 1fr;
    }
    .form-group.span-2 {
      grid-column: span 1;
    }
  }

  label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--text-primary);
  }

  input, textarea, select {
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-primary);
    padding: var(--spacing-md);
    font-size: var(--font-size-sm);
    font-family: inherit;
    color: var(--text-primary);
    background: var(--bg-primary);
    transition: var(--transition-normal);
    outline: none;
    resize: vertical;
    width: 100%;
    box-sizing: border-box;
  }

  input:focus, textarea:focus, select:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--shadow-glow);
    transform: translateY(-1px);
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
    padding: var(--spacing-md) var(--spacing-lg);
    border-radius: var(--radius-lg);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    font-family: inherit;
    cursor: pointer;
    transition: var(--transition-normal);
    border: none;
    outline: none;
    min-height: 44px;
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
    padding: var(--spacing-md);
    border-radius: var(--radius-lg);
    font-weight: var(--font-weight-medium);
    text-align: center;
    margin: var(--spacing-sm) 0;
    animation: slideDown 0.3s ease-out;
  }

  .message-banner.success {
    background: var(--success-light);
    color: var(--success);
    border: 1px solid var(--success);
  }

  .message-banner.error {
    background: var(--error-light);
    color: var(--error);
    border: 1px solid var(--error);
  }

  .shortcut-recorder {
      display: flex;
      gap: var(--spacing-md);
      align-items: center;
  }

  .shortcut-recorder input {
      flex-grow: 1;
      background-color: var(--bg-tertiary) !important;
      font-family: 'Courier New', monospace;
      font-weight: var(--font-weight-medium);
  }

  .shortcut-recorder button {
      flex-shrink: 0;
      min-width: 80px;
  }

  .about-section {
    margin-top: var(--spacing-xl);
    padding: var(--spacing-xl) !important;
    border-top: 1px solid var(--border-primary);
  }

  .about-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .about-content p {
    margin: 0;
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    line-height: 1.6;
  }

  .about-content strong {
    color: var(--text-primary);
    font-weight: var(--font-weight-semibold);
  }

  .about-content a {
    color: var(--primary);
    text-decoration: none;
    transition: all var(--transition-fast);
    border-radius: var(--radius-sm);
    padding: 2px 4px;
    margin: -2px -4px;
  }

  .about-content a:hover {
    color: var(--primary-hover);
    text-decoration: underline;
    background: var(--bg-tertiary);
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
