<template>
    <div v-show="shouldRender">
        <div class="columns">
            <span class="column is-11"></span>
            <router-link :to="'/add-serie'" class="column mdi mdi-plus-box has-text-current"> Add serie</router-link>
        </div>
        <div class="box">
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
    </div>
</template>

<script setup lang="ts">
    import { onMounted, ref } from 'vue'
    import Serie from '@/types/serie'
    import { SearchService } from '@/services/'

    const searchService = new SearchService()
    const shouldRender = ref(false)
    var series: Serie[]
    onMounted(async () => {
        searchService.searchSeries().then((response) => {
            shouldRender.value = true
            series = response as Serie[]
        })
    })
</script>
