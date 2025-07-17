<template>
    <section class="section"/>
    <section class="section"/>
    <div class="columns is-centered">
        <span class="column is-one-quarter" />
        <div class="column">
            <div v-if="shouldRender">
                <div class="card">
                    <div class="card-content">
                        <div class="media">
                        <div class="media-left">
                            <figure class="image is-48x48">
                            <img
                                src="https://bulma.io/assets/images/placeholders/96x96.png"
                                alt="Placeholder image"
                            />
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-4">{{ serie.name }}</p>
                            <!-- <p class="subtitle is-6">@johnsmith</p> -->
                        </div>
                        </div>

                        <div class="content">
                            <table class="table">
                                <thead>
                                    <tr>
                                        <th>Season</th>
                                        <th>Chapter</th>
                                        <th>Score</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td>{{ serie.season }}</td>
                                        <td>{{ serie.chapter }}</td>
                                        <td>{{ serie.score }}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                    </div>
            </div>
            <p v-else>No data</p>
        </div>
        <span class="column is-one-quarter" />
    </div>
</template>

<script setup>
    import ky from 'ky';
    import { useRoute, useRouter } from 'vue-router';
    import { onMounted, ref } from 'vue';

    const route = useRoute()
    const router = useRouter()
    const shouldRender = ref(false)
    var serie = {}
    onMounted(async () => {
        await ky.get(
            'http://127.0.0.1:8085/serie/' + route.params.serieId, 
            {
                credentials: 'include'
            }
        ).json()
        .then((data) => {
            shouldRender.value = true
            serie = data
        })
        .catch((error) => {
            if (error.response.status === 401) {
                router.push('login')
            }
        })
    })
</script>