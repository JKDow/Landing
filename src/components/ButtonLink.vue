<script setup>
import { defineProps, computed } from 'vue'
import Icon from './Icon.vue'
import { useLink } from '@/composables/useLink.js'

const colorMap = {
    red: {
        bg: 'bg-red-300',
        shadow: 'shadow-red-200/30',
    },
    blue: {
        bg: 'bg-blue-300',
        shadow: 'shadow-blue-200/30',
    },
    yellow: {
        bg: 'bg-yellow-200',
        shadow: 'shadow-yellow-200/30',
    },
    pink: {
        bg: 'bg-pink-400',
        shadow: 'shadow-pink-300/30',
    },
    green: {
        bg: 'bg-emerald-200',
        shadow: 'shadow-green-100/30',
    },
    purple: {
        bg: 'bg-purple-400',
        shadow: 'shadow-purple-200/30',
    },
    white: {
        bg: 'bg-zinc-100',
        shadow: 'shadow-zinc-50/30',
    },
}

const props = defineProps({
    color: {
        type: String,
        default: 'red',
        validator: (value) => ['white', 'red', 'blue', 'yellow', 'pink', 'green', 'purple'].includes(value),
    },
    name: {
        type: String,
        default: '',
    },
    href: {
        type: String,
        default: '',
    },
})

const { openLink } = useLink();

// Combine bg and shadow classes
const color = computed(() => {
    const map = colorMap[props.color] || colorMap.red; // Fallback to 'red'
    return `${map.bg} ${map.shadow}`;
});

const iconPath = computed(() => {
    return `icons/${props.name}.svg`;
});

function open() {
    openLink(props.href, { name: props.name, color: props.color, icon: iconPath.value });
}
</script>

<template>
    <button :class="color" @click="open"
        class="p-2 lg:p-4 gap-2 lg:gap-4 w-full h-full rounded-lg shadow-md flex flex-col items-center justify-center lg:hover:scale-105 transform transition duration-300">
        <div class="grid place-items-center h-full w-full">
            <template v-if="name.length > 0">
                <Icon :path="iconPath" size="75" />
            </template>
        </div>
        <div class="w-full flex justify-center">
            <p class="text-2xl">{{ name }}</p>
        </div>
    </button>
</template>
