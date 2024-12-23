<script setup>
import { ref } from 'vue'

const open = ref(false)

function toggle() {
    open.value = !open.value
}

function onEnter(el) {
    el.style.height = '0';
    el.offsetHeight; // Trigger reflow
    el.style.transition = 'height 0.2s ease';
    el.style.height = el.scrollHeight + 'px';
}

function onLeave(el) {
    el.style.height = el.scrollHeight + 'px';
    el.offsetHeight; // Trigger reflow
    el.style.transition = 'height 0.2s ease';
    el.style.height = '0';
}
</script>

<template>
    <div class="border border-gray-300 shadow-md rounded-md bg-stone-50 w-fit px-2 py-1">
        <button class="w-full border-gray-300 flex flex-row gap-2" :class="{ 'border-b': open }" @click="toggle">
            <h1 class="text-2xl font-bold">
                <slot name="heading">Default</slot>
            </h1>
            <i class="fas mt-1 self-center" :class="open ? 'fa-chevron-up' : 'fa-chevron-down'"></i>
        </button>
        <transition name="collapse" @enter="onEnter" @leave="onLeave">
            <div v-if="open" class="mt-2 overflow-hidden transition-[height] duration-300 ease-in-out">
                <slot name="content"></slot>
            </div>
        </transition>
    </div>
</template>
