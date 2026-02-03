import { defineConfig } from 'vite';
import path from 'path';

export default defineConfig({
  root: 'dist/3d',
  base: './',

  server: {
    port: 3000,
    open: true,
    proxy: {
      '/ws': {
        target: 'ws://localhost:8080',
        ws: true,
      },
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },

  build: {
    outDir: '../../build',
    emptyOutDir: true,
    sourcemap: true,

    rollupOptions: {
      output: {
        manualChunks: {
          three: ['three'],
        },
      },
    },

    // Optimize for production
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,
      },
    },
  },

  resolve: {
    alias: {
      '@': path.resolve(__dirname, './dist/3d/js'),
      '@cache': path.resolve(__dirname, './dist/3d/js/cache'),
      '@rendering': path.resolve(__dirname, './dist/3d/js/rendering'),
      '@entities': path.resolve(__dirname, './dist/3d/js/entities'),
    },
  },

  optimizeDeps: {
    include: ['three'],
  },
});
