<script setup>
import GoogleSearch from '../components/GoogleSearch.vue'
import QuickAccess from '../components/QuickAccess.vue'
import WindowSelector from '../components/WindowSelector.vue'
import Card from '../components/Card.vue'
import WindowManager from '@/components/WindowManager.vue'
</script>

<template>
    <div class="grid grid-cols-[2fr_5fr_2fr] grid-rows-[1fr_2fr] h-full w-full">
        <!-- Empty cell -->
        <div class=""></div>
        <!-- Search Bar -->
        <Card outer_class="justify-center" height="25%" width="70%">
            <GoogleSearch />
        </Card>
        <!-- Scroll -->
        <div class="h-full w-full pr-4">
            <Card outer_class="justify-end" height="70%" width="40%" padding="p-2">
                <WindowSelector />
            </Card>
        </div>
        <!-- Content -->
        <WindowManager />
        <!-- Side Access -->
        <Card outer_class="row-span-full col-start-3 justify-center" height="80%" width="50%">
            <QuickAccess />
        </Card>
    </div>
</template>
