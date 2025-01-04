<script setup>
import { ref } from 'vue';

const blocks = ref([
    { id: 1, color: 'bg-red-400' },
    { id: 2, color: 'bg-blue-400' },
    { id: 3, color: 'bg-yellow-400' },
    { id: 4, color: 'bg-green-400' },
    { id: 5, color: 'bg-purple-400' },
])

const spares = ref([
    { id: 6, color: 'bg-pink-400' },
    { id: 7, color: 'bg-indigo-400' },
    { id: 8, color: 'bg-cyan-400' },
    { id: 9, color: 'bg-amber-400' },
    { id: 10, color: 'bg-lime-400' },
])

const direction = ref('left');

function shiftLeft() {
    direction.value = 'left';
    spares.value.push(blocks.value.shift());
    blocks.value.push(spares.value.shift());
}

function shiftRight() {
    direction.value = 'right';
    spares.value.unshift(blocks.value.pop());
    blocks.value.unshift(spares.value.pop());
}
</script>

<template>
    <div class="h-full w-full bg-stone-100 rounded-lg p-3 flex flex-col items-center justify-center gap-4">
        <div class="flex flex-row gap-4">
            <button class="w-20 h-20 rounded-lg shadow-md bg-black/20" @click="shiftLeft">
                <i class="fa-solid fa-arrow-left"></i>
            </button>
            <button class="w-20 h-20 rounded-lg shadow-md bg-black/20" @click="shiftRight">
                <i class="fa-solid fa-arrow-right"></i>
            </button>
        </div>
        <div class="flex flex-row gap-4 items-center relative">
            <transition-group :name="direction">
                <div v-for="block in blocks" :key="block.id" :class="['w-10 h-10 lg:w-20 lg:h-20 rounded-lg shadow-md', block.color]">
                </div>
            </transition-group>
        </div>
    </div>
</template>

<style scoped>
.left-move,
.left-enter-active,
.left-leave-active,
.right-move,
.right-enter-active,
.right-leave-active {
    transition: transform 0.3s, opacity 0.3s;
}

.left-leave-active,
.right-leave-active {
    position: absolute;
}

.left-leave-to,
.right-enter-from {
    transform: translateX(-50px) scale(0.8);
    opacity: 0;
}

.left-enter-from,
.right-leave-to {
    transform: translateX(50px) scale(0.8);
    opacity: 0;
}

.left-enter-to,
.left-leave-from,
.right-enter-to,
.right-leave-from {
    transform: translateX(0) scale(1);
    opacity: 1;
}
</style>
