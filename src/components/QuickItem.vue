<script setup>
import { computed } from 'vue';
import { useLink } from '@/composables/useLink';
import Icon from './Icon.vue';

const props = defineProps({
    link: {
        type: Object,
        default: () => ({}),
    },
})

const { openQuickLink } = useLink();

const display = computed(() => {
    return props.link.name || props.link.url || '----';
});

const color = computed(() => {
    const map = {
        red: 'bg-red-300 shadow-red-200/30',
        blue: 'bg-blue-300 shadow-blue-200/30',
        yellow: 'bg-yellow-200 shadow-yellow-200/30',
        pink: 'bg-pink-400 shadow-pink-300/30',
        green: 'bg-emerald-200 shadow-green-100/30',
        purple: 'bg-purple-400 shadow-purple-200/30',
        white: 'bg-zinc-100 shadow-zinc-50/30',
    }[props.link.color] || 'bg-slate-300 shadow-slate-100/30';
    return map;
});
</script>

<template>
    <button @click="openQuickLink(link)" :class="[color]"
        class="px-2 rounded-lg shadow-md w-full flex justify-start items-center gap-2 hover:scale-[102%]">
        <div v-if="link.icon" class="pr-2 border-r border-black h-full grid place-items-center">
            <Icon :path="link.icon" />
        </div>
        <p class="grow text-center py-1">
            {{ display }}
        </p>
    </button>
</template>

<style scoped></style>
