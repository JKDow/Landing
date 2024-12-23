<script setup>
import Counter from '../components/Counter.vue'
import GridCard from '../components/GridCard.vue'
</script>

<template>
    <GridCard>
        <template #heading>
            Counter
        </template>
        <template #content>
            <Counter />
        </template>
    </GridCard>
</template>
