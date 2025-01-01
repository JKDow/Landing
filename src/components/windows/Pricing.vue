<script setup>
import { ref } from 'vue';

const blocks = ref([
    { id: 1, color: 'bg-red-400' },
    { id: 2, color: 'bg-blue-400' },
    { id: 3, color: 'bg-yellow-400' },
    { id: 4, color: 'bg-green-400' },
    { id: 5, color: 'bg-purple-400' },
])

function shiftLeft() {
    blocks.value.push(blocks.value.shift());
}

function shiftRight() {
    blocks.value.unshift(blocks.value.pop());
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
        <div class="flex flex-row gap-4 items-center">
            <div v-for="block in blocks" :key="block.id" :class="['w-20 h-20 rounded-lg shadow-md', block.color]">
            </div>
        </div>
    </div>
</template>

<script>
export default {
    title: "Blocks",
    startActive: false,
}
</script>
