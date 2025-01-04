<script setup>
import { useWindowManager } from '@/composables/useWindowManager';
import Card from '@/components/Card.vue';
import WindowSelector from '@/components/WindowSelector.vue';
import { defineProps, computed, ref } from 'vue';

const { windowList, ready, totalWindows, getCircularIndex, activeIndex, direction } = useWindowManager();

const visibleWindows = computed(() => {
    if (!ready.value) return [];
    const range = totalWindows.value >= 3 ? 3 : 1;
    const result = [];
    for (let i = -2; i <= 0; i++) {
        const circularIndex = getCircularIndex(activeIndex.value + i);
        if (windowList.value[circularIndex]) {
            result.push({
                ...windowList.value[circularIndex],
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
    <transition-group :name="`windows-${direction}`" tag="div"
        class="max-lg:justify-self-end w-full h-full max-h-full max-w-full relative grow">
        <div v-for="(win, index) in visibleWindows" :key="win.id" :id="`contentCard-${win.id}`"
            :class="['w-full h-full absolute transform-gpu will-change-transform', getClass(win.offset)]">
            <Card outer_class="justify-center" inner_class="flex flex-col gap-2 w-full h-full"
                :active="win.offset === 0" height="85%" width="90%">
                <component :is="win.component" />
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
