<template>
    <br>
    <div class="columns is-centered">
        <div class="column is-three-quarters-fullhd is-four-fifths content">
            <Header v-if="shouldRender" />
            <section class="section" v-else/>
            <RouterView :key="$route.fullPath" />
        </div>
    </div>
</template>

<style>
    @import "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css";
    @import "https://cdn.jsdelivr.net/npm/@mdi/font@7.4.47/css/materialdesignicons.min.css";
    @import "./static/css/custom.css";
</style>

<script setup lang="ts">
    import Header from '@/components/Header.vue'
    import sessionStore from './stores/session-store'
    import { useRouter } from 'vue-router'
    import { ref, watch } from 'vue'

    const router = useRouter()
    const session = sessionStore()
    const shouldRender = ref(false)

    function getRenderValue() {
        if (session.token == null) {
            shouldRender.value = false
            router.push({ name: 'login' })
        } else {
            shouldRender.value = true
        }
    }

    getRenderValue()

    watch(session, () => getRenderValue())
</script>
