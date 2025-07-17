<template>
    <section class="section"/>
    <section class="section"/>
    <section class="section"/>
    <div class="columns is-centered is-vcentered">
        <span class="column is-two-fifths"></span>
        <div class="column">
            <div class="field">
                <p class="control has-icons-left has-icons-right">
                    <input v-model="username" class="input" type="text" placeholder="Username">
                    <span class="icon is-small is-left">
                    <i class="mdi mdi-email-outline"></i>
                    </span>
                    <span class="icon is-small is-right">
                    <i class="mdi mdi-check"></i>
                    </span>
                </p>
            </div>
            <div class="field">
                <p class="control has-icons-left">
                    <input v-model="password" class="input" type="password" placeholder="Password">
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
        <span class="column is-two-fifths"></span>
    </div>
</template>

<script setup>
    import { ref } from 'vue';
    import sessionStore from '@/stores/session-store';

    const session = sessionStore();
    const username = ref()
    const password = ref()

    async function login() {
        session.login(username?.value, password?.value)
    }
</script>
