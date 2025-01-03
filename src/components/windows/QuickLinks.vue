<script setup>
import ButtonLink from '@/components/ButtonLink.vue'
import Clock from '@/components/Clock.vue'
</script>

<template>
    <div class="w-full h-full grid grid-cols-2 lg:grid-cols-5 lg:grid-rows-3 gap-4 max-lg:overflow-scroll">
        <ButtonLink color="blue" name="Github" href="https://github.com" />
        <ButtonLink color="yellow" name="Google" href="https://google.com" />
        <ButtonLink color="pink" name="Gmail" href="https://mail.google.com" />
        <ButtonLink color="purple" name="AWS" href="https://aws.amazon.com/" />
        <ButtonLink color="green" name="ChatGPT" href="https://chatgpt.com" />
        <ButtonLink color="pink" name="Laravel" href="https://laravel.com/" />
        <div class="w-full h-full col-span-2 lg:col-span-3 max-lg:row-start-1">
            <div class="bg-zinc-100 shadow-zinc-50/30 p-2 w-full h-full rounded-lg shadow-md">
                <Clock />
            </div>
        </div>
        <ButtonLink color="red" name="Rust" href="https://www.rust-lang.org/learn" />
        <ButtonLink color="purple" name="Udemy" href="https://www.udemy.com/" />
        <ButtonLink color="blue" name="Jira" href="https://www.atlassian.com/software/jira" />
        <ButtonLink color="red" name="YouTube" href="https://www.youtube.com" />
        <ButtonLink color="green" name="Figma" href="https://www.figma.com/" />
        <ButtonLink color="yellow" name="DevDocs" href="https://devdocs.io/" />
    </div>
</template>

<script>
    export default {
        title: "Quick Links",
        startActive: true,
    }
</script>

