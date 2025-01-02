<script setup>
import manager from '@/composables/useWindowManager';
import Card from '@/components/Card.vue';
import { defineProps, computed, ref } from 'vue';

const visibleWindows = computed(() => {
    if (!manager.ready) return [];
    const range = manager.totalWindows >= 3 ? 3 : 1;
    const result = [];
    for (let i = -2; i <= 0; i++) {
        const circularIndex = manager.getCircularIndex(manager.activeIndex + i);
        if (manager.windows[circularIndex]) {
            result.push({
                ...manager.windows[circularIndex],
                offset: i,
            });
        }
    }
    return result;
});

// Offset is from -2 to 0
function getClass(offset) {
    return `offset-${Math.abs(offset)}`;
}
</script>

<template>
    <transition-group :name="`windows-${manager.direction}`" tag="div"
        class="w-full h-full max-h-full max-w-full relative">
        <div v-for="(window, index) in visibleWindows" :key="window.id" :id="`contentCard-${window.id}`"
            :class="['w-full h-full absolute', getClass(window.offset)]">
            <Card outer_class="justify-center" inner_class="flex flex-col gap-2 w-full h-full"
                :active="window.offset === 0" height="85%" width="90%">
                <component :is="window.component" />
            </Card>
        </div>
    </transition-group>
</template>

<style scoped>
.absolute {
    transition: transform 0.5s ease-in-out, z-index 0.5s ease-in-out;
}

.offset-2 {
    transform: translateY(20px) translateX(-20px);
    z-index: 0;
}

.offset-1 {
    transform: translateY(10px) translateX(-10px);
    z-index: 10;
}

.offset-0 {
    transform: translateY(0) translateX(0);
    z-index: 20;
}

.windows-up-enter-active,
.windows-up-leave-active,
.windows-down-enter-active,
.windows-down-leave-active {
    transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1),
        opacity 0.5s ease-in-out,
        z-index 0.5s ease-in-out;
}

.windows-down-enter-from,
.windows-up-leave-to {
    transform: translateX(30px) translateY(-30px) scale(1);
    opacity: 0;
}

.windows-up-enter-from,
.windows-down-leave-to {
    transform: translateX(-30px) translateY(30px) scale(1);
    opacity: 0;
}
</style>
