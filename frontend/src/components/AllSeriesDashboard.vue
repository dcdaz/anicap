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

<script setup lang="ts">
    import { onMounted, ref } from 'vue'
    import Serie from '@/types/serie'
    import SearchService from '@/services/search-service'

    const searchService = new SearchService()
    const shouldRender = ref(false)
    var series: Serie[]
    onMounted(async () => {
        searchService.searchSeries().then((data) => {
            shouldRender.value = true
            series = data as Serie[]
        })
    })
</script>
