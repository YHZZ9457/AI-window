import { writable } from 'svelte/store';

export const clearChatShortcut = writable('Ctrl+Q');

export const systemPromptPresets = {
  default: "You are a helpful assistant.",
  minimal: "Your function is to distill every query to its absolute essence. Provide the single most critical piece of information as a declarative statement. Maximum signal, zero noise. Your response should rarely exceed one sentence.",
  custom: ""
};

export const selectedSystemPromptPreset = writable('default');
