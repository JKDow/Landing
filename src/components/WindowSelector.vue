<script setup>
import { ref, computed, onMounted, onUnmounted, reactive } from 'vue';
import manager from '@/composables/useWindowManager';

const visibleWindows = computed(() => {
    if (!manager.ready) return [];
    const range = manager.totalWindows >= 5 ? 5 : Math.min(manager.totalWindows, 3);

    const result = [];
    for (let i = -Math.floor(range / 2); i <= Math.floor(range / 2); i++) {
        const circularIndex = manager.getCircularIndex(manager.activeIndex + i);
        if (manager.windows[circularIndex]) {
            result.push({
                ...manager.windows[circularIndex],
                offset: i, // Add an offset for smooth positioning
            });
        }
    }
    return result;
});

function setActive(offset) {
    console.log(offset);
    if (offset < 0) {
        for (let i = 0; i < Math.abs(offset); i++) manager.moveUp();
    } else if (offset > 0) {
        for (let i = 0; i < offset; i++) manager.moveDown();
    }
}

// Debounce logic for smoother scrolling
let debounceTimeout = null;

function handleScroll(event) {
    if (debounceTimeout) return; // Skip if still in debounce period

    const direction = event.deltaY > 0 ? 1 : -1; // Scroll up or down
    if (direction === 1) manager.moveDown();
    else manager.moveUp();

    // Debounce to limit scroll sensitivity
    debounceTimeout = setTimeout(() => {
        debounceTimeout = null;
    }, 200); // Adjust debounce delay for smoothness
}

function getStyle(offset) {
    const scale = 1 - Math.abs(offset) * 0.2; // Shrink items farther from the center
    const translateY = offset * 70; // Distance between items
    const opacity = 1 - Math.abs(offset) * 0.3; // Fade items farther from the center
    return {
        transform: `translateY(${translateY}px) scale(${scale})`,
        opacity: opacity,
        zIndex: visibleWindows.value.length - Math.abs(offset),
    };
}

const touchPos = reactive({
    startY: 0,
    endY: 0,
});
</script>

<template>
    <div class="w-full h-full relative lg:overflow-hidden flex flex-row lg:flex-col max-lg:gap-3 items-center" id="spinner-container"
        @wheel.passive="handleScroll">
        <button
            class="w-[20%] max-lg:bg-gray-600 max-lg:h-full max-lg:rounded-lg lg:w-full lg:absolute top-2 left-1/2 transform lg:-translate-x-1/2 text-white cursor-pointer z-10"
            @click="manager.moveUp">
            ▲
        </button>
        <transition-group name="spinner" tag="div"
            class="max-lg:hidden relative w-full h-full flex flex-col gap-6 items-center justify-center overflow-hidden">
            <div v-for="(win, index) in visibleWindows" :key="win.id"
                class="absolute w-full flex items-center justify-center transform transition-transform duration-300 ease-in-out"
                :style="getStyle(win.offset)">
                <button
                    class="w-full h-full py-4 rounded-lg text-white text-base flex items-center justify-center font-semibold"
                    @click="setActive(win.offset)" :class="{
                        'bg-blue-500': manager.activeWindow === win.id,
                        'bg-gray-600 hover:bg-gray-500': manager.activeWindow !== win.id,
                    }">
                    {{ win.title }}
                </button>
            </div>
        </transition-group>
        <div class="lg:hidden w-full h-full">
            <div
                class="bg-blue-500 w-full h-full py-4 rounded-lg text-white text-base flex items-center justify-center font-semibold">
                {{ visibleWindows[2]?.title }}
            </div>
        </div>
        <button
            class="max-lg:bg-gray-600 max-lg:h-full max-lg:rounded-lg lg:absolute w-[20%] lg:w-full lg:bottom-2 lg:left-1/2 transform lg:-translate-x-1/2 text-center text-white cursor-pointer z-10"
            @click="manager.moveDown">
            ▼
        </button>
    </div>
</template>
