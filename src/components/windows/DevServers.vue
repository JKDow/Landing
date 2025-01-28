<script setup>
import { ref, onMounted, computed } from 'vue';

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

onMounted(() => {
    scanForServers();
});

const scanForServers = async () => {
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
            <div v-for="server in servers" :key="server"
                class="flex items-center gap-3 p-4 rounded-lg shadow-md bg-gray-800">
                <span class="w-4 h-4 rounded-full indicator" :class="online.includes(server) ? 'online' : 'offline'"
                    title="Server status"></span>
                <a v-if="online.includes(server)" class="text-lg font-medium text-green-400 hover:underline"
                    :href="server" target="_blank" rel="noopener noreferrer">
                    {{ server }}
                </a>
                <p v-else class="text-lg font-medium text-red-400">
                    {{ server }}
                </p>
            </div>
        </div>
        <p class="text-xl font-semibold text-gray-400">
            Online Servers: {{ online.length }} / {{ servers.length }}
        </p>
    </div>
</template>

<style scoped>
.indicator {
    position: relative;
    display: inline-block;
}

.online {
    background: #4caf50;
    box-shadow: 0 0 6px rgba(76, 175, 80, 0.8);
}

.offline {
    background: #f44336;
    box-shadow: 0 0 6px rgba(244, 67, 54, 0.8);
}

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
