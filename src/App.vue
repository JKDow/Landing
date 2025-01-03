<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
    <div class="h-dvh w-dvw max-w-dvw max-h-dvh flex flex-col bg-stone-800 overflow-clip">
        <canvas id="background" class="absolute top-0 left-0 pointer-events-none z-0"></canvas>
        <!-- Navigation -->
        <header class="hidden w-full h-24 z-10 p-3">

        </header>

        <main class="p-3 grow w-full z-10 relative">
            <RouterView />
        </main>
    </div>
</template>
