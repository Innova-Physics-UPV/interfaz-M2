// @ts-check
import { defineConfig } from 'astro/config';

import svelte from '@astrojs/svelte';

// https://astro.build/config
export default defineConfig({
  integrations: [svelte()],
  output: 'static', 
  build: {
    // Opcional: asegura que los assets se generen donde Tauri los espera
    format: 'file'
  }
});
