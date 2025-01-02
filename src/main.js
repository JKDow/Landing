import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import './index.css'

const app = createApp(App)
app.use(router)
app.mount('#app')

import init, { NightSky } from './wasm/wasm.js';

import manager from '@/composables/useWindowManager';
manager.initializeWindows();

// Initialize WASM after the Vue app is mounted
async function run() {
    await init();
    const canvas = document.getElementById('background');
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    const nightSky = await new NightSky(canvas, "#252525", 2500);

    const resizeCanvas = () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        nightSky.resize(window.innerWidth, window.innerHeight);
    };

    window.addEventListener('resize', resizeCanvas);

    let lastTime = performance.now();

    function render() {
        const currentTime = performance.now();
        const deltaTime = (currentTime - lastTime) / 1000; // Convert to seconds
        lastTime = currentTime;
        nightSky.update_and_render(deltaTime);
        requestAnimationFrame(render);
    }
    render();
}

run().catch((err) => {
    console.error('Failed to initialize WASM:', err);
});


/*
const particleSystem = new ParticleSystem('background', 0.001, 4, 50);

function render() {
    particleSystem.update_and_render();
    requestAnimationFrame(render);
}

render();
*/
