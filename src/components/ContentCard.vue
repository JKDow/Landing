<script setup>
import { ref, defineProps } from 'vue'
</script>

<template>
    <div v-bind="$attrs" class="w-full h-full">
        <slot></slot>
    </div>
</template>
