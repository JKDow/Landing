<script setup>
import { useLink } from '@/composables/useLink.js'

const props = defineProps({
    online: {
        type: Boolean,
        default: false,
    },
    server: {
        type: String,
        default: '',
    },
})

const { openLink } = useLink();

function open() {
    if (!props.online) return;
    // get port out of url
    const port = props.server.split(':')[2].split('/')[0];
    openLink(props.server, { icon: 'icons/Localhost.svg', name: `Local:${port}` });
}
</script>

<template>
    <div class="flex items-center gap-3 p-4 rounded-lg shadow-md bg-gray-800">
        <span class="w-4 h-4 rounded-full indicator" :class="online ? 'online' : 'offline'"
            title="Server status"></span>
        <button v-if="online" @click="open" class="text-lg font-medium text-green-400 hover:underline" :href="server">
            {{ server }}
        </button>
        <p v-else class="text-lg font-medium text-red-400">
            {{ server }}
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
</style>
