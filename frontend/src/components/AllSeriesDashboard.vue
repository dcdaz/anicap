<template>
    <div v-if="shouldRender" class="box">
        <table class="table is-fullwidth is-striped">
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
                    <td>
                        <router-link :to="`/serie-dashboard/${serie.id}`" class="has-text-current">{{ serie.name }}</router-link>
                    </td>
                    <td>{{ serie.season }}</td>
                    <td>{{ serie.chapter }}</td>
                    <td>{{ serie.score }}</td>
                </tr>
            </tbody>
        </table>
    </div>
    <p v-else class="has-text-centered">No data</p>
</template>

<script setup>
    import ky from 'ky'
    import { useRouter } from 'vue-router'
    import { inject, onMounted, ref } from 'vue'

    const baseUrl = inject('baseUrl')
    const router = useRouter()
    const shouldRender = ref(false)
    var series = []
    onMounted(async () => {
        await ky.get(
            `${baseUrl}/serie`,
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
                router.push({ name: 'login' })
            }
        })
    })
</script>
