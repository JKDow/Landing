import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import './index.css'

const app = createApp(App)
app.use(router)
app.mount('#app')

import { useWindowManager } from '@/composables/useWindowManager';
const { initializeWindows } = useWindowManager();
initializeWindows('Home').catch(console.error);

import { useStarSystem } from '@/composables/useStarSystem';
const { setup } = useStarSystem();
setup().catch(console.error);

