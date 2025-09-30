<script>
  import '../app.css';
  import { theme, applyTheme, watchSystemTheme } from '$lib/stores/theme';
  import { onMount } from 'svelte';
  import { _ } from 'svelte-i18n';

  onMount(() => {
    // Apply initial theme
    const unsubscribe = theme.subscribe(currentTheme => {
      applyTheme(currentTheme);
    });

    // Watch for system theme changes
    const cleanup = watchSystemTheme();

    return () => {
      unsubscribe();
      cleanup();
    };
  });
</script>

<svelte:head>
  <title>{$_('app.title')}</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="">
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet">
</svelte:head>

<div class="app-container">
  <nav class="sidebar">
    <div class="sidebar-header">
      <h1>{$_('app.title')}</h1>
    </div>
    <ul>
      <li><a href="/">{$_('nav.home')}</a></li>
      <li><a href="/settings">{$_('nav.settings')}</a></li>
    </ul>
  </nav>
  <main class="main-content">
    <slot />
  </main>
</div>

<style>
  .app-container {
    display: flex;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  .sidebar {
    width: 240px;
    background-color: var(--bg-secondary);
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    border-right: 1px solid var(--border-primary);
  }

  .sidebar-header h1 {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .sidebar ul {
    list-style: none;
    padding: 0;
    margin-top: 2rem;
  }

  .sidebar ul li a {
    display: block;
    padding: 0.75rem 1rem;
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    text-decoration: none;
    font-weight: 500;
    transition: background-color 0.2s, color 0.2s;
  }

  .sidebar ul li a:hover {
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-lg);
  }
</style>
