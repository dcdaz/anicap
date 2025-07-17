<template>
    <RouterView />
</template>

<style>
    @import "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css";
    @import "https://cdn.jsdelivr.net/npm/@mdi/font@7.4.47/css/materialdesignicons.min.css";
</style>

<script setup>
    import sessionStore from '@/stores/session-store';
    import { useRouter } from 'vue-router'

    const router = useRouter()
    const session = sessionStore();

    if(session.token == null) {
        router.push('login')
    }
    router.push('all-series-dashboard')
</script>
