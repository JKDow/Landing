<script setup>
import manager from '@/composables/useWindowManager';
import Card from '@/components/Card.vue';
import { defineProps, onMounted, computed, ref } from 'vue';

const visibleWindows = computed(() => {
    const range = manager.totalWindows >= 3 ? 3 : 1;
    const result = [];
    for (let i = -3; i <= 1; i++) {
        const circularIndex = manager.getCircularIndex(manager.activeIndex + i);
        if (manager.windows[circularIndex]) {
            result.push({
                ...manager.windows[circularIndex],
                offset: i,
                show: i !== -3 && i !== 1,
            });
        }
    }
    return result;
});

onMounted(() => {
    manager.initializeWindows();
});

// Offset is from -2 to 0
function getStyle(offset) {
    const translate = offset * 10;
    return {
        transform: `translateY(${-translate}px) translateX(${translate}px)`,
        zIndex: 20 + offset * 10,
    };
}
</script>

<template>
    <div class="w-full h-full max-h-full max-w-full relative">
        <div v-for="(window, index) in visibleWindows" :key="window.id" :id="`contentCard-${window.id}`"
            :class="['w-full h-full absolute', window.show ? '' : 'pointer-events-none']">
            <Card outer_class="justify-center" :style="getStyle(window.offset)" :active="window.offset === 0"
                height="85%" width="90%" v-show="window.show">
                <div class="flex flex-col gap-2 w-full h-full">
                    <component :is="window.component" />
                </div>
            </Card>
        </div>
    </div>
</template>
