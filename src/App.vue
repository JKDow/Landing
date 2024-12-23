<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
    <!-- Navigation -->
    <header class="hidden">
        <div class="w-full bg-red-200">
            <nav>
                <RouterLink to="/">Home</RouterLink>
            </nav>
        </div>
    </header>

    <RouterView />
</template>
