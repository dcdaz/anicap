<template>
    <header class="block">
        <nav class="navbar is-transparent" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <router-link :to="'/'">
                    <span class="has-text-info title mdi mdi-television-classic"> {{ appTitle }}</span>
                </router-link>
                <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="headerMenu">
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                </a>
            </div>
            <div id="headerMenu" class="navbar-menu">
                <div class="navbar-start">
                    <span class="navbar-item"></span>
                </div>
                <div class="navbar-end">
                    <div class="navbar-item">
                        <div class="field is-grouped">
                            <a id="theme-switcher" class="mdi mdi-theme-light-dark has-text-current" @click="toggleTheme"></a>
                            <a class="mdi mdi-github has-text-current" href="https://github.com/dcdaz/anicap"></a>
                            <a class="mdi mdi-logout has-text-current" @click="logout"></a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    </header>
</template>

<script setup lang="ts">
    import toggleTheme from '@/utils/theme-switcher'
    import sessionStore from '@/stores/session-store'
    import navbarBurgerMenu from '@/utils/navbar'
    import { onMounted } from 'vue'

    const appTitle = process.env.VUE_APP_TITLE

    onMounted(() => {
        navbarBurgerMenu()
    })

    async function logout() {
        sessionStore().logout()
    }
</script>