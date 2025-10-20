import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// 创建一个简单的可写存储
function createPersistentStore<T>(key: string, defaultValue: T) {
  const { subscribe, set, update } = writable<T>(defaultValue);

  // 初始化时从后端加载值
  invoke('get_settings').then((settings: any) => {
    if (settings && settings[key] !== undefined) {
      set(settings[key]);
    }
  }).catch(() => {
    // 如果加载失败，使用默认值
    set(defaultValue);
  });

  return {
    subscribe,
    set: (value: T) => {
      set(value);
      // 保存到后端
      invoke('get_settings').then((currentSettings: any) => {
        const updatedSettings = { ...currentSettings, [key]: value };
        invoke('set_settings', { settings: updatedSettings });
      }).catch(console.error);
    },
    update,
  };
}

export const clearChatShortcut = createPersistentStore('clear_chat_shortcut', 'Ctrl+Q');

export const systemPromptPresets = {
  default: 'You are a helpful assistant. Please answer the user\'s questions concisely.',
  minimal: 'Your function is to distill every query to its absolute essence. Provide the single most critical piece of information as a declarative statement. Maximum signal, zero noise. Your response should rarely exceed one sentence.',
  custom: ''
};

export const selectedSystemPromptPreset = createPersistentStore('selectedSystemPromptPreset', 'default');

export const apiKey = createPersistentStore('api_key', '');
export const apiUrl = createPersistentStore('api_url', 'https://api.openai.com/v1/chat/completions');
export const modelName = createPersistentStore('model_name', 'gpt-4o-mini');
export const shortcut = createPersistentStore('shortcut', 'Alt+Space');
export const systemPrompt = createPersistentStore('system_prompt', 'You are a helpful assistant. Please answer the user\'s questions concisely.');
export const apiType = createPersistentStore('api_type', 'openai');
export const borderless = createPersistentStore('borderless', false);

export const borderlessShortcut = createPersistentStore('borderless_shortcut', 'Ctrl+Shift+B');