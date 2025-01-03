<script setup>
import GoogleSearch from '../components/GoogleSearch.vue'
import QuickAccess from '../components/QuickAccess.vue'
import WindowSelector from '../components/WindowSelector.vue'
import Card from '../components/Card.vue'
import WindowManager from '@/components/WindowManager.vue'
</script>

<template>
    <div
        class="flex max-lg:justify-start max-lg:items-center max-lg:gap-4 flex-col lg:grid lg:grid-cols-[2fr_5fr_2fr] lg:grid-rows-[1fr_2fr] h-full w-full">
        <!-- Empty cell -->
        <div class=""></div>
        <!-- Search Bar -->
        <Card outer_class="justify-center max-lg:h-[8%]" height="25%" width="70%">
            <GoogleSearch />
        </Card>
        <!-- Scroll -->
        <Card outer_class="justify-center max-lg:h-[10%] lg:justify-end lg:-translate-x-[20px]" height="70%" width="40%"
            padding="p-2">
            <WindowSelector />
        </Card>
        <!-- Content -->
        <div class="w-[90%] max-lg:grow max-lg:pb-8 max-lg:justify-self-end lg:h-full lg:w-full">
            <WindowManager />
        </div>
        <!-- Side Access -->
        <Card outer_class="row-span-full lg:col-start-3 justify-center max-lg:hidden" height="80%" width="50%">
            <QuickAccess />
        </Card>
    </div>
</template>
