<template>
    <div v-if="shouldRender">
        <div class="card">
            <div class="card-content">
                <div class="content">
                    <table class="table is-fullwidth is-striped">
                        <thead class="has-text-centered">
                            <tr>
                                <th>
                                    Serie Type
                                    <input class="input editable is-small has-text-centered" type="number"/>
                                </th>
                                <th>
                                    Serie Genre
                                    <input class="input editable is-small has-text-centered" type="number"/>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="type in types" :key="type.id">
                                <td>{{ type.name }}</td>
                            </tr>
                            <tr v-for="genre in genres" :key="genre.id">
                                <td>{{ genre.name }}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
            <footer class="card-footer has-text-centered">
                <a class="card-footer-item mdi mdi-arrow-left-circle has-text-current" @click="$router.go(-1)"> Back</a>
                <a class="card-footer-item has-text-current">
                    <p @click=""><span class="mdi mdi-content-save"></span> Save</p>
                </a>
            </footer>
        </div>
    </div>
    <p v-else class="has-text-centered">No data</p>
</template>

<script setup lang="ts">
    import { onMounted, Ref, ref } from 'vue'
    import SerieGenre from '@/types/serie-genre'
    import SerieType from '@/types/serie-type'
    import SerieSettingsService from '@/services/serie-settings-service'

    const serieSettingsService = new SerieSettingsService()
    const genres: Ref<SerieGenre[]> = ref([])
    const types: Ref<SerieType[]> = ref([])

    const shouldRender = ref(false)

    onMounted(async () => {
        serieSettingsService.getSerieGenres().then(response => genres.value = response)
        shouldRender.value = true
    })
</script>