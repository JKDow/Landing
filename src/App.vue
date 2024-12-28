<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
    <div class="h-dvh w-dvw flex flex-col bg-stone-800">
        <canvas id="background" class="absolute top-0 left-0 pointer-events-none z-0"></canvas>
        <!-- Navigation -->
        <header class="hidden w-full h-fit bg-stone-900 text-white p-3">
            <div class="w-full flex flex-row justify-center items-center">
                <nav>
                    <RouterLink to="/">Home</RouterLink>
                </nav>
            </div>
        </header>

        <main class="p-3 grow w-full z-10 relative">
            <RouterView />
        </main>
    </div>
</template>
