<script setup>
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { useEventBus } from '@/composables/useEventBus.js';
import ServerLink from '@/components/ServerLink.vue';

const lastChecked = ref(null);
const online = ref([]);
const loading = ref(false);
const servers = [
    'http://localhost:3000/',
    'http://localhost:8000/',
    'http://localhost:8080/',
    'http://localhost:5000/',
    'http://localhost:4200/',
    'http://localhost:5173/'
];

const { on, off } = useEventBus();

onMounted(() => {
    scanForServers();
    on('scanServers', scanForServers);
});

onUnmounted(() => {
    off('scanServers', scanForServers);
});

const scanForServers = async () => {
    if (loading.value) return; // Prevent multiple scans at once
    online.value = []; // Reset the server list
    loading.value = true;

    await Promise.all(
        servers.map((url) =>
            fetch(url, { method: 'HEAD', mode: 'no-cors' }) // Try fetching the root
                .then((response) => {
                    online.value.push(url);
                })
                .catch(() => { })
        )
    );
    loading.value = false;
    lastChecked.value = new Date().toLocaleTimeString();
};
</script>

<template>
    <div class="w-full h-full flex flex-col gap-6 p-6 text-gray-300 overflow-auto">
        <div class="flex flex-row justify-between">
            <h2 class="text-4xl font-bold text-white">Server Status</h2>
            <div class="flex flex-row items-end gap-1">
                <div class="px-2 mt-1 flex items-end">
                    <div v-if="loading" class="loader"></div>
                    <i v-else class="fa-solid fa-arrows-rotate text-white rounded-lg cursor-pointer"
                        @click="scanForServers"></i>
                </div>
                <p class="text-xs text-gray-500">Last checked: {{ lastChecked }}</p>
            </div>
        </div>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
            <ServerLink v-for="server in servers" :key="server" :online="online.includes(server)" :server />
        </div>
        <p class="text-xl font-semibold text-gray-400">
            Online Servers: {{ online.length }} / {{ servers.length }}
        </p>
    </div>
</template>

<style scoped>
.loader {
    border: 4px solid rgba(255, 255, 255, 0.3);
    border-top: 4px solid #4caf50;
    border-radius: 50%;
    width: 18px;
    height: 18px;
    padding-right: 4px;
    animation: spin 0.8s linear infinite;
}

@keyframes spin {
    0% {
        transform: rotate(0deg);
    }

    100% {
        transform: rotate(360deg);
    }
}
</style>
