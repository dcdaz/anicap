<template>
    <section class="section"/>
    <div class="columns is-centered is-vcentered">
        <span class="column is-one-fifth"></span>
        <div class="column">
            <div class="field">
                <p class="control has-icons-left has-icons-right">
                    <input v-model="username" class="input" type="text" placeholder="Username" @keyup.enter="login">
                    <span class="icon is-small is-left">
                        <i class="mdi mdi-account-outline"></i>
                    </span>
                    <span class="icon is-small is-right">
                        <i class="mdi mdi-check"></i>
                    </span>
                </p>
            </div>
            <div class="field">
                <p class="control has-icons-left">
                    <input v-model="password" class="input" type="password" placeholder="Password" @keyup.enter="login">
                    <span class="icon is-small is-left">
                        <i class="mdi mdi-lock-outline"></i>
                    </span>
                </p>
            </div>
            <div class="field">
                <p class="control">
                    <button class="button is-success" @click="login">
                        Login
                    </button>
                </p>
            </div>
        </div>
        <span class="column is-one-fifth"></span>
    </div>
</template>

<script setup lang="ts">
    import { ref } from 'vue'
    import sessionStore from '@/stores/session-store'

    const session = sessionStore();
    const username = ref()
    const password = ref()

    async function login() {
        session.login(username?.value, password?.value)
    }
</script>
