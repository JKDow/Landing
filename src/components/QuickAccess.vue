<script setup>
import { ref } from 'vue';
import QuickItem from './QuickItem.vue';
import { useLink } from '@/composables/useLink.js';

const { linkHistory, clearHistory, openLink } = useLink();
</script>

<template>
    <div class="flex flex-col gap-4 w-full h-full">
        <h2 class="text-2xl font-bold text-white text-center">Quick Access</h2>
        <div class="flex flex-col items-center w-full h-full gap-3 grow">
            <QuickItem v-for="(link, index) in linkHistory" :key="index" :link />
        </div>
        <button @click="clearHistory"
            class="text-gray-400 py-2 px-4 w-full text-right hover:scale-105">
            Clear History
        </button>
    </div>
</template>
