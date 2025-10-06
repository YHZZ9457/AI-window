import { writable } from 'svelte/store';

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
