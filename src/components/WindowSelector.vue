<script setup>
import { ref } from 'vue'
import { useWindowManager } from '@/composables/useWindowManager'
const { state, setActiveWindow } = useWindowManager()

function setActive(id) {
    setActiveWindow(id);
}

</script>

<template>
    <div class="w-full h-full flex flex-col justify-center gap-2">
        <!-- Dynamically list the windows -->
        <button v-for="window in state.windows" :key="window.id" class="px-4 py-2 rounded-lg text-white text-sm" :class="{
            'bg-blue-500': state.activeWindow === window.id,
            'bg-gray-600 hover:bg-gray-500': state.activeWindow !== window.id,
        }" @click="setActive(window.id)">
            {{ window.title }}
        </button>
    </div>
</template>
