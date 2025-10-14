<script lang="ts">
  import { marked } from 'marked';
  import { onMount, tick } from 'svelte';

  let { text } = $props<{ text: string }>();

  let html = $state<string>('');

  async function renderMarkdown() {
    html = await marked.parse(text);
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

<div class="prose">
  {@html html}
</div>

<style>
  .prose {
    all: revert;
  }
</style>