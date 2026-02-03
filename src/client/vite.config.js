import { defineConfig } from "vite";
import path from "path";
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
    root: "public",
    base: "./",

    server: {
        port: 3000,
        open: true,
        proxy: {
            "/ws": {
                target: "ws://localhost:8080",
                ws: true,
            },
            "/api": {
                target: "http://localhost:8080",
                changeOrigin: true,
            },
        },
    },

    build: {
        outDir: "../dist",
        emptyOutDir: true,
        sourcemap: true,

        rollupOptions: {
            output: {
                manualChunks: {
                    three: ["three"],
                },
            },
        },

        // Optimize for production
        minify: "terser",
        terserOptions: {
            compress: {
                drop_console: true,
            },
        },
    },

    resolve: {
        alias: {
            "/src": path.resolve(__dirname, "./src"),
            "@": path.resolve(__dirname, "./src"),
            "@cache": path.resolve(__dirname, "./src/cache"),
            "@rendering": path.resolve(__dirname, "./src/rendering"),
            "@entities": path.resolve(__dirname, "./src/entities"),
            "@utils": path.resolve(__dirname, "./src/utils"),
            "@assets": path.resolve(__dirname, "./assets"),
        },
    },

    optimizeDeps: {
        include: ["three"],
    },

    publicDir: "../assets",
});
