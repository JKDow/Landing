import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import { execSync } from 'child_process';

function gitCommitDatePlugin() {
    return {
        name: 'inject-git-date',
        config() {
            const date = execSync('git log -1 --format=%cd').toString().trim();
            process.env.VITE_LAST_UPDATED = date;
        },
    };
}

// https://vite.dev/config/
export default defineConfig({
    base: '/Landing',
    plugins: [
        vue(),
        vueDevTools(),
        wasm(),
        gitCommitDatePlugin(),
    ],
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url))
        },
    },
})
