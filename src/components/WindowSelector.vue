<script setup>
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { useWindowManager } from '@/composables/useWindowManager';

const { state, setActiveWindow, ready } = useWindowManager();

const position = ref(0); // Tracks the current center position

// Total windows available
const totalWindows = computed(() => state.windows.length);

// Circularly get the index for infinite scrolling
const getCircularIndex = (index) => {
    return (index + totalWindows.value) % totalWindows.value;
};

// Visible windows for the spinner
const visibleWindows = computed(() => {
    let range;
    if (totalWindows.value >= 5) range = 5;
    else if (totalWindows.value >= 3) range = 3;
    else range = 1;

    const result = [];
    for (let i = -Math.floor(range / 2); i <= Math.floor(range / 2); i++) {
        const circularIndex = getCircularIndex(position.value + i);
        if (state.windows[circularIndex]) {
            result.push({
                ...state.windows[circularIndex],
                offset: i, // Add an offset for smooth positioning
            });
        }
    }
    return result;
});

// Update the active window and center position
function setActive(id) {
    const index = state.windows.findIndex((window) => window.id === id);
    if (index !== -1) {
        position.value = index;
        setActiveWindow(id);
    }
}

// Debounce logic for smoother scrolling
let debounceTimeout = null;

function handleScroll(event) {
    if (debounceTimeout) return; // Skip if still in debounce period

    const direction = event.deltaY > 0 ? 1 : -1; // Scroll up or down
    position.value = getCircularIndex(position.value + direction);
    setActive(state.windows[getCircularIndex(position.value)].id);

    // Debounce to limit scroll sensitivity
    debounceTimeout = setTimeout(() => {
        debounceTimeout = null;
    }, 200); // Adjust debounce delay for smoothness
}

watch(
    ready,
    (isReady) => {
        if (isReady) {
            const activeIndex = state.windows.findIndex((window) => window.id === state.activeWindow);
            if (activeIndex !== -1) {
                position.value = activeIndex;
            }
        }
    },
    { immediate: true }
);

function getStyle(offset) {
    const scale = 1 - Math.abs(offset) * 0.2; // Shrink items farther from the center
    const translateY = offset * 70; // Distance between items
    const opacity = 1 - Math.abs(offset) * 0.3; // Fade items farther from the center

    return `
        transform: translateY(${translateY}px) scale(${scale});
        opacity: ${opacity};
        z-index: ${visibleWindows.value.length - Math.abs(offset)}; /* Center item has the highest z-index */
    `;
}

onMounted(() => {
    // find Id spinner-container
    const spinnerContainer = document.getElementById('spinner-container');
    if (spinnerContainer) {
        spinnerContainer.addEventListener('wheel', handleScroll, { passive: true });
    }
});

onUnmounted(() => {
    // Clean up the wheel event listener
    const spinnerContainer = document.getElementById('spinner-container');
    if (spinnerContainer) {
        spinnerContainer.removeEventListener('wheel', handleScroll);
    }
});
</script>

<template>
    <div class="w-full h-full relative overflow-hidden flex items-center" id="spinner-container">
        <button class="w-full absolute top-2 left-1/2 transform -translate-x-1/2 text-white cursor-pointer z-10"
            @click="handleScroll({ deltaY: -1 })">
            ▲
        </button>
        <transition-group name="spinner" tag="div"
            class="relative w-full h-full flex flex-col gap-6 items-center justify-center overflow-hidden">
            <div v-for="(window, index) in visibleWindows" :key="window.id"
                class="absolute w-full flex items-center justify-center transform transition-transform duration-300 ease-in-out"
                :style="getStyle(window.offset)" @click="setActive(window.id)">
                <button
                    class="w-full h-full py-4 rounded-lg text-white text-base flex items-center justify-center font-semibold"
                    :class="{
                        'bg-blue-500': state.activeWindow === window.id,
                        'bg-gray-600 hover:bg-gray-500': state.activeWindow !== window.id,
                    }">
                    {{ window.title }}
                </button>
            </div>
        </transition-group>
        <button class="absolute w-full bottom-2 left-1/2 transform -translate-x-1/2 text-center text-white cursor-pointer"
            @click="handleScroll({ deltaY: 1 })">
            ▼
        </button>
    </div>
</template>
