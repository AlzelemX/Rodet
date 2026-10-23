import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';

// The editor UI lives in ui/. The desktop app (app/) and devr-server both
// load the built files from ui/dist.
export default defineConfig({
  root: 'ui',
  resolve: {
    // Monaco's ESM files by path (its package exports hide most of them).
    alias: { 'monaco-esm': fileURLToPath(new URL('./node_modules/monaco-editor/esm/vs', import.meta.url)) },
  },
  base: './',
  clearScreen: false,
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    target: 'es2022',
    chunkSizeWarningLimit: 6000,
    sourcemap: false,
  },
  worker: { format: 'es' },
  server: {
    port: 5173,
    strictPort: true,
  },
});
