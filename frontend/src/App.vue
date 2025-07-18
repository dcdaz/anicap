<template>
    <div class="columns">
        <span class="column is-one-fifth" />
        <div class="column">
            <Header v-if="shouldRender" :is="Header" />
            <section class="section" v-if="!shouldRender"/>
            <RouterView :key="$route.fullPath" />
        </div>
        <span class="column is-one-fifth" />
    </div>
</template>

<style>
    @import "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css";
    @import "https://cdn.jsdelivr.net/npm/@mdi/font@7.4.47/css/materialdesignicons.min.css";
    @import "./css/custom.css";
</style>

<script setup>
    import Header from './components/Header.vue'
    import sessionStore from '@/stores/session-store'
    import { useRouter } from 'vue-router'
    import { ref } from 'vue'

    const router = useRouter()
    const session = sessionStore()
    const shouldRender = ref(false)

    if(session.token == null) {
        router.push({ name: 'login' })
    } else {
        shouldRender.value = true
        router.push({ name: 'home' })
    }
</script>
