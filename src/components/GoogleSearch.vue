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
    <div class="flex justify-center items-center bg-gray-800 w-full h-full px-2 py-1 rounded-xl">
        <form class="flex flex-row justify-center w-full h-full text-xl px-2" @submit.prevent="handleSearch">
            <input class="bg-transparent focus:outline-none grow text-white" v-model="query" type="text" placeholder="Search Google or type a URL" />
            <button type="submit" class="ml-2">
                <i class="fas fa-search text-gray-500"></i>
            </button>
        </form>
    </div>
</template>
