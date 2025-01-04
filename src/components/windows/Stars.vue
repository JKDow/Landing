<script setup>
import { ref } from "vue";
import { useStarSystem } from "@/composables/useStarSystem";
const { starCount, addStars, removeStars, starSystem } = useStarSystem();

const isAdvanced = starSystem.value.is_advanced();

const incrementAmount = ref(500);
const decrementAmount = ref(500);
</script>

<template>
    <div class="h-full w-full flex flex-col gap-2">
        <div class="grid grid-cols-2 gap-3 h-full w-full">
            <div
                class="border h-full w-full rounded-md border-gray-300/50 bg-stone-900 p-2 text-stone-100 flex flex-col gap-1 overflow-y-scroll">
                <h2 class="text-2xl w-full text-center">Info</h2>
                <p class="">These are the controls for the Star System background</p>
                <p>
                    All background animations are handled in Web Assembly and are written in the Rust programming
                    language
                </p>
                <p>
                    Depending on the hardware the browser is run on the background will prefer to run on the GPU using
                    a WGPU Pipeline.
                </p>
                <p>
                    When using CPU rendering the number and size of the stars will be limited to prevent performance
                    issues.
                    On GPU rendering I have tested up to 1M stars running.
                </p>
            </div>
            <div
                class="grid grid-cols-2 grid-rows-4 gap-2 text-white border size-full rounded-md border-gray-300/50 bg-stone-900 p-2 overflow-y-scroll lg:overflow-y-clip">
                <div class="text-sm lg:text-base 4xl:text-xl bg-stone-700 p-3 rounded-md flex flex-col items-center justify-center gap-3">
                    <input type="range" min="100" max="2000" step="100" v-model.number="incrementAmount"
                        class="bg-gray-600 appearance-none w-full h-2 rounded-lg" />
                    <button
                        class="size-full bg-gray-700 hover:bg-gray-800 rounded-lg grid grid-cols-[40%_60%] grid-rows-2 place-items-center"
                        @click="addStars(incrementAmount)">
                        <i class="fa-solid fa-plus mr-2 row-span-2"></i>
                        <p class="leading-4">Add Stars</p>
                        <p>( {{ incrementAmount }} )</p>
                    </button>
                </div>
                <div class="text-sm lg:text-base 4xl:text-xl bg-stone-700 p-3 rounded-md flex flex-col items-center justify-center gap-3">
                    <input type="range" min="100" max="2000" step="100" v-model.number="incrementAmount"
                        class="bg-gray-600 appearance-none w-full h-2 rounded-lg" />
                    <button
                        class="size-full bg-gray-700 hover:bg-gray-800 rounded-lg grid grid-cols-[40%_60%] grid-rows-2 place-items-center"
                        @click="removeStars(decrementAmount)">
                        <i class="fa-solid fa-minus row-span-2"></i>
                        <p class="leading-4">Remove Stars</p>
                        <p>( {{ decrementAmount }} )</p>
                    </button>
                </div>
                <div class="col-span-2 bg-slate-200 text-stone-900 w-full rounded-lg p-2 flex flex-col justify-center">
                    <p class="text-sm font-medium w-full text-center">Current Stars</p>
                    <p class="text-2xl font-semibold w-full text-center">{{ starCount }}</p>
                </div>
                <div class=" bg-stone-700 grid place-items-center rounded-md col-span-2">
                    <p>{{ isAdvanced ? 'Rendering On GPU' : 'Rendering On CPU' }}</p>
                </div>
            </div>
        </div>
    </div>
</template>
