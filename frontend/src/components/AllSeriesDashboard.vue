<template>
    <section class="section"/>
    <section class="section"/>
    <div class="columns">
        <span class="column is-one-quarter" />
        <div class="column box">
            <div v-if="shouldRender">
                <table class="table">
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Season</th>
                            <th>Chapter</th>
                            <th>Score</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="serie in series" :key="serie.id">
                            <td><router-link  :to="'/serie-dashboard/' + serie.id">{{ serie.name }}</router-link></td>
                            <td>{{ serie.season }}</td>
                            <td>{{ serie.chapter }}</td>
                            <td>{{ serie.score }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <p v-else>No data</p>
        </div>
        <span class="column is-one-quarter" />
    </div>
</template>

<script setup>
    import ky from 'ky';
    import { useRouter } from 'vue-router';
    import { onMounted, ref } from 'vue';

    const router = useRouter()
    const shouldRender = ref(false)
    var series = []
    onMounted(async () => {
        await ky.get(
            'http://127.0.0.1:8085/serie', 
            {
                credentials: 'include'
            }
        ).json()
        .then((data) => {
            shouldRender.value = true
            series = data
        })
        .catch((error) => {
            if (error.response.status === 401) {
                router.push('login')
            }
        })
    })
</script>
