import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig({
  plugins: [sveltekit()],
  clearScreen: false,
  server: {
    port: 15173,
    strictPort: true,
    host: host || false,
  },
});
