import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import './index.css'

import init, { ParticleSystem } from './wasm/wasm.js';

const app = createApp(App)
app.use(router)
app.mount('#app')

// Initialize WASM after the Vue app is mounted
init().then(() => {
    console.log("Wasm loaded");
    console.log("App started");
    const canvas = document.getElementById('background');

    const resizeCanvas = () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
    };
    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);

    const particleSystem = new ParticleSystem('background', 0.001, 4, 50);

    function render() {
        particleSystem.update_and_render();
        requestAnimationFrame(render);
    }

    render();
}).catch((err) => {
    console.error('Failed to initialize WASM:', err);
});
