import { writable } from 'svelte/store';
import { _ } from 'svelte-i18n';

export type Attachment = {
  name: string;
  content: string;
};

export type Message = {
  role: 'user' | 'assistant' | 'system';
  content: string;
  attachment?: Attachment | null;
};

function createChatStore() {
  const { subscribe, set, update } = writable<Message[]>([]);
  let initialMessage = 'AI response will appear here.'; // Default fallback

  // Subscribe to the store from svelte-i18n to get the translation function
  const unsubscribe = _.subscribe(t => {
    try {
      const translated = t('home.initialMessage');
      if (translated && translated !== 'home.initialMessage') {
        initialMessage = translated;
      }
    } catch (error) {
      // i18n not ready yet, use default
    }
  });

  const init = () => {
    update(messages => {
      if (messages.length === 0) {
        return [{ role: 'assistant', content: initialMessage }];
      }
      return messages;
    });
  };

  const clearChat = () => {
    set([{ role: 'assistant', content: initialMessage }]);
  };

  return {
    subscribe,
    set,
    update,
    init, // Expose the init function
    clearChat,
    addUserMessage: (content: string, attachment: Attachment | null = null) => {
      const newMessage: Message = { role: 'user', content, attachment };
      update(messages => {
        const currentMessages = messages;
        if (currentMessages.length === 1 && currentMessages[0].role === 'assistant') {
          return [newMessage];
        }
        return [...currentMessages, newMessage];
      });
    },
    addAssistantMessage: (content: string) => {
      update(messages => [...messages, { role: 'assistant', content }]);
    },
  };
}

export const chat = createChatStore();