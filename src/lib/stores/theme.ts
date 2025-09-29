import { writable } from 'svelte/store';

export type Theme = 'light' | 'dark' | 'auto';

// Initialize theme from localStorage or default to 'auto'
const storedTheme = typeof localStorage !== 'undefined' ? localStorage.getItem('theme') as Theme : null;
const initialTheme: Theme = storedTheme || 'auto';

export const theme = writable<Theme>(initialTheme);

// Function to get the actual theme (resolves 'auto' to system preference)
export function getActualTheme(currentTheme: Theme): 'light' | 'dark' {
  if (currentTheme === 'auto') {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
  return currentTheme;
}

// Function to apply theme to document
export function applyTheme(currentTheme: Theme) {
  const actualTheme = getActualTheme(currentTheme);
  document.documentElement.setAttribute('data-theme', actualTheme);
  
  // Store in localStorage
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem('theme', currentTheme);
  }
}

// Listen for system theme changes
export function watchSystemTheme() {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  
  const handleChange = () => {
    theme.update(current => {
      if (current === 'auto') {
        applyTheme(current);
      }
      return current;
    });
  };
  
  mediaQuery.addEventListener('change', handleChange);
  
  return () => {
    mediaQuery.removeEventListener('change', handleChange);
  };
}