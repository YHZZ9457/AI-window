import { writable } from 'svelte/store';

type Message = {
  role: 'user' | 'assistant' | 'system';
  content: string;
};

function createChatStore() {
  const { subscribe, set, update } = writable<Message[]>([]);

  return {
    subscribe,
    set,
    update,
    clearChat: (initialContent: string) => {
      set([{ role: 'assistant', content: initialContent }]);
    },
    setInitial: (initialContent: string) => {
        update(messages => {
            if (messages.length === 0) {
                return [{ role: 'assistant', content: initialContent }];
            }
            return messages;
        });
    },
    addUserMessage: (content: string) => {
      update(messages => {
        const currentMessages = messages;
        if (currentMessages.length === 1 && currentMessages[0].role === 'assistant') {
            return [{ role: 'user', content }];
        }
        return [...currentMessages, { role: 'user', content }];
      });
    },
    addAssistantMessage: (content: string) => {
      update(messages => [...messages, { role: 'assistant', content }]);
    },
  };
}

export const chat = createChatStore();
