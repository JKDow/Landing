<script setup>
import { ref, computed, onMounted } from 'vue';
import FlipNum from './FlipNum.vue';

const currentTime = ref(new Date());
const clockSize = ref(50);

onMounted(() => {
    setClockSize();
    window.addEventListener('resize', setClockSize);
    setInterval(() => {
        currentTime.value = new Date();
    }, 1000);
});

function setClockSize() {
    const width = window.innerWidth;
    if (width < 1280) {
        clockSize.value = 30;
    } else if (width < 1900) {
        clockSize.value = 40;
    } else if (width < 2100) {
        clockSize.value = 50;
    } else if (width < 2600) {
        clockSize.value = 90;
    }
}

// Formats to loacl 12h time and creates index for each digit
const datetime = computed(() => {
    const hours = currentTime.value.getHours();
    const hours12 = hours % 12 || 12;
    const minutes = currentTime.value.getMinutes();
    const date = currentTime.value.getDate();
    const day = currentTime.value.getDay();
    return {
        h1: Math.floor(hours12 / 10),
        h2: hours12 % 10,
        m1: Math.floor(minutes / 10),
        m2: minutes % 10,
        ampm: hours >= 12 ? 'PM' : 'AM',
        date: date,
        day: day,
    };
});

const ampmClass = "px-1 lg:px-2 lg:py-1 grid place-items-center text-center";
const amClass = computed(() => {
    return [
        datetime.value.ampm === 'AM' ? 'bg-blue-400 text-stone-900' : 'bg-gray-300 text-gray-400',
        ampmClass
    ];
});
const pmClass = computed(() => {
    return [
        datetime.value.ampm === 'PM' ? 'bg-blue-400 text-stone-900' : 'bg-gray-300 text-gray-400',
        ampmClass
    ];
});

// list of days with current day in the middle
const days = computed(() => {
    const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    const currentDate = currentTime.value;

    const getDayInfo = (offset) => {
        const adjustedDate = new Date(currentDate);
        adjustedDate.setDate(currentDate.getDate() + offset);
        return {
            day: dayNames[adjustedDate.getDay()],
            date: adjustedDate.getDate(),
        };
    };
    return [
        ...[-3, -2, -1].map(getDayInfo),
        getDayInfo(0),
        ...[1, 2, 3].map(getDayInfo),
    ];
});
</script>

<template>
    <div class="w-full h-full flex flex-col justify-between items-center gap-2">
        <!-- Time Display -->
        <div class="text-4xl font-mono flex flex-row gap-2 w-full justify-center items-center">
            <FlipNum :number="datetime.h1" :size="clockSize" />
            <FlipNum :number="datetime.h2" :size="clockSize" />
            <p class="font-semibold">:</p>
            <FlipNum :number="datetime.m1" :size="clockSize" />
            <FlipNum :number="datetime.m2" :size="clockSize" />
            <div class="flex h-[80%] text-base lg:text-lg font-semibold rounded-xl overflow-hidden">
                <div :class="[amClass, ampmClass]">AM</div>
                <div :class="[pmClass, ampmClass]">PM</div>
            </div>
        </div>
        <!-- Date Display -->
        <div class="w-full max-w-full flex flex-row gap-2 justify-center px-2">
            <div v-for="day in days" :key="day.date"
                class="flex flex-col basis-[14%] items-center border border-stone-400 rounded-lg p-1 lg:p-2 cursor-default shadow-sm hover:shadow-lg transition-shadow">
                <div class="text-sm font-medium text-gray-600" v-text="day.day"></div>
                <div class="mt-1 px-2 flex items-center justify-center rounded-full text-lg min-[2500px]:text-2xl font-semibold"
                    :class="day.date === datetime.date ? 'bg-blue-600 text-white' : 'bg-gray-100 text-gray-800'"
                    v-text="day.date"></div>
            </div>
        </div>
    </div>
</template>
