<script setup>
import { ref } from 'vue'
const count = ref(0)

function increment () {
    count.value++;
}
</script>

<template>
    <button
        class="p-3 bg-gray-200 text-semibold rounded-full border border-gray-800 w-full"
        @click="increment">
        {{ count }}
    </button>
</template>
