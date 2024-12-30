<script setup>
import { useWindowManager } from '@/composables/useWindowManager';
import { defineProps, onMounted } from 'vue';
const { state, registerWindow } = useWindowManager();

const windowData = [
    {
        id: 'QuickLinks',
        title: 'Quick Links',
        active: true,
    },
    {
        id: 'Info',
        title: 'Info',
    },
];

onMounted(() => {
    windowData.forEach((window) => {
        registerWindow(window);
    });
});

</script>

<template>
    <div v-for="window in state.windows" :key="window.id" v-show="window.id === state.activeWindow"
        class="w-full h-full">
        <component :is="window.component" />
    </div>
</template>
