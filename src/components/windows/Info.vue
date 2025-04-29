<script setup>
import { useLink } from '@/composables/useLink.js';

const { newTab } = useLink();

function openRepo() {
    newTab('https://github.com/JKDow/Landing');
}
</script>

<template>
    <div class="w-full h-full text-white flex flex-col gap-4 items-center text-center px-2 relative">
        <h1 class="text-3xl font-bold mt-3">Info</h1>
        <div class="flex flex-col gap-1 mt-2 items-center">
            <p class="">
                This is a personal project I created to learn Vue 3.
            </p>
            <p class="">
                I also wanted to connect WASM to it for the background.
            </p>
        </div>
        <div class="flex flex-col gap-1 mt-2 items-center">
            <p>
                The project was written using Vue 3 and Vite was used to compile it to a static site to host on Github
                Pages.
            </p>
            <p>
                The background star system simulation is written in Rust and uses WGPU to render the particles
                ('Stars').
            </p>
        </div>
        <div class="flex flex-col gap-1 mt-2 items-center">
            <p>
                The default setting is 10,000 star particles, but you can increase it to 100,000 or even 1,000,000.
            </p>
            <p>
                On my personal computer, I can run 1,000,000 particles at 60 FPS.
            </p>
        </div>
        <p>
            The purpose is not to be a high performance simulation but rather a fun background I could make by learning
            the basics of GPU programming.
        </p>
        <button class="absolute bottom-0 right-0 px-4 text-xl hover:text-gray-300 hover:underline" @click="openRepo">
            Repo
            <i class="fa-brands fa-github"></i>
        </button>
    </div>
</template>
