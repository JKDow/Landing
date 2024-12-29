<script setup>
import Counter from '../components/Counter.vue'
import GoogleSearch from '../components/GoogleSearch.vue'
import QuickAccess from '../components/QuickAccess.vue'
import ContentCard from '../components/ContentCard.vue'
import ScrollSelector from '../components/ScrollSelector.vue'
import Card from '../components/Card.vue'
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
        <Card outer_class="justify-end" height="70%" width="40%">
            <ScrollSelector />
        </Card>
        <!-- Content -->
        <Card outer_class="justify-center" height="90%" width="95%">
            <ContentCard />
        </Card>
        <!-- Side Access -->
        <Card outer_class="row-span-full col-start-3 justify-center" height="80%" width="50%">
            <QuickAccess />
        </Card>
    </div>
</template>
