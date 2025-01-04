<script setup>
import { useStarSystem } from "@/composables/useStarSystem";

const { addStars, removeStars, starCount } = useStarSystem();

const boxStyle = "border h-full w-full rounded-md border-gray-300/50 bg-stone-900 p-2";
const headingStyle = "text-lg font-semibold";
</script>

<template>
    <div class="grid grid-cols-2 grid-rows-2 gap-2 h-full w-full text-gray-200">
        <div :class="['row-span-2', boxStyle]">
            <h1 :class="headingStyle">Settings</h1>
        </div>
        <div :class="boxStyle">
        </div>
        <div :class="boxStyle">
            <h1 :class="headingStyle">Resources</h1>
            <ul class="list-disc pl-5">
                <li class="font-medium">
                    Company Icons:
                    <a class="underline hover:text-blue-500 font-normal" href="https://simpleicons.org/">Simple Icons</a>
                </li>
                <li class="font-medium">
                    Other Icons:
                    <a class="underline hover:text-blue-500 font-normal" href="https://fontawesome.com/icons">Font Awesome</a>
                </li>
            </ul>
        </div>
    </div>
</template>

<script>
export default {
    title: "Settings",
    startActive: false,
}
</script>
