<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
    <div class="h-dvh w-dvw flex flex-col">
        <!-- Navigation -->
        <header class="w-full h-fit bg-stone-900 text-white p-3">
            <div class="w-full flex flex-row justify-center items-center">
                <nav>
                    <RouterLink to="/">Home</RouterLink>
                </nav>
            </div>
        </header>

        <main class="p-3 bg-stone-100 grow w-full">
            <RouterView />
        </main>
    </div>
</template>
