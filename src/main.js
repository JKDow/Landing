import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import './index.css'

const app = createApp(App)
app.use(router)
app.mount('#app')

import manager from '@/composables/useWindowManager';
manager.initializeWindows();

import { useStarSystem } from '@/composables/useStarSystem';
const { setup } = useStarSystem();
await setup();

