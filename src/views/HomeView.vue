<script setup>
import Counter from '../components/Counter.vue'
import GoogleSearch from '../components/GoogleSearch.vue'
import GridCard from '../components/GridCard.vue'
import QuickAccess from '../components/QuickAccess.vue'
import ContentCard from '../components/ContentCard.vue'
import ScrollSelector from '../components/ScrollSelector.vue'
</script>

<template>
    <div class="grid grid-cols-[2fr_5fr_2fr] grid-rows-[1fr_2fr] h-full w-full">
        <!-- Empty cell -->
        <div class=""></div>
        <!-- Search Bar -->
        <div class="flex items-center justify-center">
            <div class="w-[50%] h-[20%]">
                <GoogleSearch />
            </div>
        </div>
        <!-- Scroll -->
        <div class="flex items-center justify-end">
            <div class="h-[70%] w-[40%]">
                <ScrollSelector />
            </div>
        </div>
        <!-- Content -->
        <div class="flex justify-center items-center">
            <div class="h-[80%] w-[80%]">
                <ContentCard>
                    <template #content>
                        <Counter />
                        <div class="" id="wasm-window">
                        </div>
                    </template>
                </ContentCard>
            </div>
        </div>
        <!-- Side Access -->
        <div class="row-span-full col-start-3 flex justify-center items-center">
            <div class="w-[50%] h-[80%]">
                <QuickAccess />
            </div>
        </div>
    </div>
</template>
