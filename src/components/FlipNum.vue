<script setup>
import { ref, watch, nextTick, computed } from 'vue';

const props = defineProps({
    number: {
        type: Number,
        required: true,
    },
    size: {
        type: Number,
        default: 50,
    },
});

const currentFace = ref(props.number);
const faces = ref([props.number, props.number, props.number, props.number]);
const translateZ = computed(() => props.size / 2);
const perspective = computed(() => props.size * 4);

const outerStyle = computed(() => {
    return {
        transform: `translateZ(${translateZ.value}px) rotateX(${90 + (currentFace.value * 90)}deg)`,
        width: `${props.size}px`,
        height: `${props.size}px`,
    };
});

watch(
    () => props.number,
    (newValue, oldValue) => {
        const newFace = newValue > oldValue
            ? (currentFace.value + 1) % 4
            : (currentFace.value - 1 + 4) % 4;
        faces.value[newFace] = newValue;
        currentFace.value = newValue;
    }
);

function getStyle(i) {
    const rotateX = i * 90;
    return {
        transform: `rotateX(${-rotateX}deg) translateZ(${translateZ.value}px)`,
        width: `${props.size}px`,
        height: `${props.size}px`,
    };
}
</script>

<template>
    <div class="" :style="{ perspective: `${perspective.value}px` }">
        <div class="w-full h-full cube relative" :style="outerStyle">
            <template v-for="i in 4" :key="i">
                <div class="cube-face grid place-items-center absolute bg-stone-800 border-black/50 border max-lg:text-xl"
                    :style="getStyle(i)">
                    <div :id="`face-${i}`" class="text-white" v-text="faces[i - 1]"></div>
                </div>
            </template>
        </div>
    </div>
</template>

<style scoped>
.cube {
    transform-style: preserve-3d;
    transition: transform 1s;
}

.cube-face {
    transition: transform 1s;
}
</style>
