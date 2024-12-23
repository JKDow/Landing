<script setup>
import { ref } from 'vue'
</script>

<template>
    <div class="bg-gray-800 w-full h-full px-2 py-1 rounded-lg">
        <slot name="content"></slot>
    </div>
</template>
