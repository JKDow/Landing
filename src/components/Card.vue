<script setup>
import { defineProps } from 'vue'

const props = defineProps({
    height: {
        type: String,
        default: '100%',
    },
    width: {
        type: String,
        default: '100%',
    },
    outer_class: {
        type: [String, Array, Object], // Support various class formats
        default: '',
    },
    inner_class: {
        type: [String, Array, Object], // Support various class formats
        default: '',
    },
    padding: {
        type: String,
        default: 'p-4',
    },
    active: {
        type: Boolean,
        default: true,
    },
})

</script>

<template>
    <div :class="['flex items-center w-full h-full', outer_class]">
        <div :class="[
            'rounded-xl border-2 border-gray-100/60 focus-within:shadow-[0_0_20px_5px_rgba(240,240,240,0.5)] transition duration-300',
            inner_class,
            padding,
            active ? 'bg-black/20 backdrop-blur-3xl backdrop-opacity-90 backdrop-brightness-80' : 'bg-transparent',
        ]" :style="{ height: props.height, width: props.width }">
            <div class="w-full h-full" v-show="active">
                <slot></slot>
            </div>
        </div>
    </div>
</template>
