<script setup>
import { useWindowManager } from '@/composables/useWindowManager';
import Card from '@/components/Card.vue';
import { defineProps, onMounted, computed } from 'vue';
const { state, initializeWindows } = useWindowManager();

// Total windows available
const totalWindows = computed(() => state.windows.length);

const position = computed(() => state.windows.findIndex((window) => window.id === state.activeWindow));

// Circularly get the index for infinite scrolling
const getCircularIndex = (index) => {
    return (index + totalWindows.value) % totalWindows.value;
};

const visibleWindows = computed(() => {
    let range;
    if (state.windows.length >= 3) range = 3;
    else range = 1;
    const result = [];
    for (let i = -(range - 1); i <= 0; i++) {
        const circularIndex = getCircularIndex(position.value + i);
        if (state.windows[circularIndex]) {
            result.push({
                ...state.windows[circularIndex],
                offset: i,
            });
        }
    }
    return result;
});

onMounted(() => {
    initializeWindows();
});

// Offset is from -2 to 0
function getStyle(offset) {
    // set z axis based on offset
    const z = 20 + offset * 10;
    // move up slightly for each -1 offset
    const translateY = offset * 5;

    return `
        transform: translateY(${translateY}px);
        z-index: ${z};
    `;
}

</script>

<template>
    <div class="w-full h-full relative">
        <template v-for="(window, index) in visibleWindows" :key="window.id" class="w-full h-full">
            <div class="absolute w-full h-full" :id="`contentCard-${window.id}`">
                <Card outer_class="justify-center" height="90%" width="95%">
                    <div class="flex flex-col gap-2 w-full h-full" :style="getStyle(window.offset)">
                        <component :is="window.component" />
                    </div>
                </Card>
            </div>
        </template>
    </div>
</template>
