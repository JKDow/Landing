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

    await Promise.allSettled(
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
                <i class="fa-solid fa-arrows-rotate px-2 mt-1 text-white rounded-lg cursor-pointer"
                    @click="scanForServers"></i>
                <p class="text-xs text-gray-500">Last checked: {{ lastChecked }}</p>
            </div>
        </div>
        <div v-if="loading" class="flex flex-col items-center justify-center h-32 gap-2">
            <div class="loader"></div>
            <p class="text-sm text-gray-400">Scanning servers...</p>
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

/* Online (Green Light) */
.online {
    background: #4caf50;
    /* Solid green color */
    box-shadow: 0 0 6px rgba(76, 175, 80, 0.8);
    /* Subtle green glow */
}

/* Offline (Red Light) */
.offline {
    background: #f44336;
    /* Solid red color */
    box-shadow: 0 0 6px rgba(244, 67, 54, 0.8);
    /* Subtle red glow */
}

/* Inner dot for the subtle light effect */
.indicator:after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 60%;
    /* Smaller dot inside the circle */
    height: 60%;
    background: rgba(255, 255, 255, 0.3);
    /* Subtle white overlay */
    border-radius: 50%;
    box-shadow: inset 0 0 2px rgba(0, 0, 0, 0.2);
    /* Light shadow for depth */
}

.loader {
    border: 4px solid rgba(255, 255, 255, 0.3);
    /* Light gray border */
    border-top: 4px solid #4caf50;
    /* Green spinner */
    border-radius: 50%;
    width: 32px;
    /* Slightly larger loader */
    height: 32px;
    animation: spin 0.8s linear infinite;
    /* Faster, smoother spin */
}

/* Spinner animation */
@keyframes spin {
    0% {
        transform: rotate(0deg);
    }

    100% {
        transform: rotate(360deg);
    }
}
</style>
