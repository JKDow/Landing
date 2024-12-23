import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
    base: '/Landing',
    plugins: [
        vue(),
        vueDevTools(),
        wasm(),
    ],
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url))
        },
    },
})
