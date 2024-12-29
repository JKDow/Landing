<script setup>
import ButtonLink from './ButtonLink.vue'
import { ref, defineProps } from 'vue'
</script>

<template>
    <div class="w-full h-full grid grid-cols-5 grid-rows-3 gap-4">
        <ButtonLink color="blue" name="Github" href="https://github.com" />
        <ButtonLink color="yellow" name="Gmail" href="https://mail.google.com" />
        <ButtonLink color="pink" />
        <ButtonLink color="purple" />
        <ButtonLink color="green" />
        <ButtonLink color="pink" />
        <div class="w-full h-full col-span-3">
            <ButtonLink color="white" />
        </div>
        <ButtonLink color="red" />
        <ButtonLink color="purple" />
        <ButtonLink color="blue" />
        <ButtonLink color="red" />
        <ButtonLink color="green" />
        <ButtonLink color="yellow" />
    </div>
</template>
