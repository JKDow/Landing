<script setup>
import { ref } from 'vue';

// Reactive query state
const query = ref('');

// Handle search form submission
function handleSearch() {
    if (query.value.trim()) {
        const searchUrl = `https://www.google.com/search?q=${encodeURIComponent(query.value)}`;
        window.open(searchUrl, '_blank'); // Open Google search in a new tab
    }
};
</script>

<template>
    <div class="flex justify-center items-center w-full h-full">
        <form class="flex flex-row justify-center w-full h-full text-2xl px-2 text-white placeholder-gray-300" @submit.prevent="handleSearch">
            <input class="bg-transparent focus:outline-none grow" v-model="query" type="text" placeholder="Search Google or type a URL" />
            <button type="submit" class="ml-2 rounded-full px-2 transition-transform duration-200 hover:scale-110">
                <i class="fas fa-search"></i>
            </button>
        </form>
    </div>
</template>
