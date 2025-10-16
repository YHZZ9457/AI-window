<script lang="ts">
  import { marked } from 'marked';
  import { onMount, tick } from 'svelte';

  let { text } = $props<{ text: string }>();

  let html = $state<string>('');

  // Configure marked options for better rendering
  marked.setOptions({
    breaks: true,
    gfm: true
  });

  // Check if this is the initial placeholder message
  const isInitialMessage = (content: string): boolean => {
    const initialMessages = [
      'AI response will appear here.',
      'AI 的回复会出现在这里。',
      'AI 的回覆會出現在這裡。',
      'AIの応答はここに表示されます。'
    ];
    return initialMessages.includes(content.trim());
  };

  async function renderMarkdown() {
    // For initial messages, don't use Markdown to avoid extra paragraph spacing
    if (isInitialMessage(text)) {
      html = text;
    } else {
      html = await marked.parse(text);
    }
  }

  onMount(async () => {
    await renderMarkdown();
  });

  $effect(() => {
    if (text) {
      (async () => {
        await tick();
        await renderMarkdown();
      })();
    }
  });
</script>

<div class="markdown-content" class:initial-message={isInitialMessage(text)}>
  {@html html}
</div>

<style>
  .markdown-content {
    line-height: 1.6;
    word-wrap: break-word;
    overflow-wrap: break-word;
  }

  .markdown-content :global(h1),
  .markdown-content :global(h2),
  .markdown-content :global(h3),
  .markdown-content :global(h4),
  .markdown-content :global(h5),
  .markdown-content :global(h6) {
    margin: 1.5em 0 0.5em 0;
    font-weight: var(--font-weight-semibold);
    line-height: 1.25;
  }

  .markdown-content :global(h1) {
    font-size: 1.5em;
    border-bottom: 1px solid var(--border-primary);
    padding-bottom: 0.3em;
  }

  .markdown-content :global(h2) {
    font-size: 1.25em;
    border-bottom: 1px solid var(--border-primary);
    padding-bottom: 0.3em;
  }

  .markdown-content :global(h3) {
    font-size: 1.125em;
  }

  .markdown-content :global(h4) {
    font-size: 1em;
  }

  .markdown-content :global(h5) {
    font-size: 0.875em;
  }

  .markdown-content :global(h6) {
    font-size: 0.85em;
    color: var(--text-secondary);
  }

  .markdown-content :global(p) {
    margin: 1em 0;
  }

  .markdown-content :global(ul),
  .markdown-content :global(ol) {
    margin: 1em 0;
    padding-left: 2em;
  }

  .markdown-content :global(li) {
    margin: 0.5em 0;
  }

  .markdown-content :global(blockquote) {
    margin: 1em 0;
    padding: 0.5em 1em;
    border-left: 4px solid var(--border-primary);
    background: var(--bg-tertiary);
    border-radius: 0 var(--radius-md) var(--radius-md) 0;
  }

  .markdown-content :global(blockquote p) {
    margin: 0;
    color: var(--text-secondary);
  }

  .markdown-content :global(code) {
    background: var(--bg-tertiary);
    padding: 0.2em 0.4em;
    border-radius: var(--radius-sm);
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 0.85em;
    color: var(--text-primary);
  }

  .markdown-content :global(pre) {
    background: var(--bg-tertiary);
    padding: 1em;
    border-radius: var(--radius-md);
    overflow-x: auto;
    margin: 1em 0;
    border: 1px solid var(--border-primary);
  }

  .markdown-content :global(pre code) {
    background: none;
    padding: 0;
    border-radius: 0;
    color: inherit;
  }

  .markdown-content :global(table) {
    border-collapse: collapse;
    width: 100%;
    margin: 1em 0;
    border: 1px solid var(--border-primary);
  }

  .markdown-content :global(th),
  .markdown-content :global(td) {
    border: 1px solid var(--border-primary);
    padding: 0.5em 1em;
    text-align: left;
  }

  .markdown-content :global(th) {
    background: var(--bg-tertiary);
    font-weight: var(--font-weight-semibold);
  }

  .markdown-content :global(a) {
    color: var(--primary);
    text-decoration: none;
  }

  .markdown-content :global(a:hover) {
    text-decoration: underline;
  }

  .markdown-content :global(img) {
    max-width: 100%;
    height: auto;
    border-radius: var(--radius-sm);
    margin: 1em 0;
  }

  .markdown-content :global(hr) {
    border: none;
    border-top: 1px solid var(--border-primary);
    margin: 2em 0;
  }

  .markdown-content :global(strong) {
    font-weight: var(--font-weight-semibold);
  }

  .markdown-content :global(em) {
    font-style: italic;
  }

  /* Special styling for initial placeholder message */
  .markdown-content.initial-message {
    margin: 0;
    padding: 0;
  }

  .markdown-content.initial-message :global(p) {
    margin: 0;
  }
</style>