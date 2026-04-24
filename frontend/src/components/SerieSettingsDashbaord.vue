<template>
    <div v-if="shouldRender">
        <div class="card">
            <div class="card-content">
                <div class="content">
                    <div class="tabs is-centered is-boxed">
                        <ul>
                            <li id="tab-type" class="is-active"><a @click="showSection('type')">Serie Type</a></li>
                            <li id="tab-genre"><a @click="showSection('genre')">Serie Genre</a></li>
                        </ul>
                    </div>
                    <div class="columns">
                        <span class="column is-one-fifth-fullhd"></span>
                        <table class="table is-fullwidth is-striped" v-show="showType">
                            <thead class="has-text-centered">
                                <tr>
                                    <th>
                                        <input id="serie-type-input" class="input editable is-small has-text-centered" type="text" @keyup.enter="addSerieType" v-model="newSerieType" placeholder="Type here to add a new Serie Type"/>
                                    </th>
                                </tr>
                            </thead>
                            <tbody class="has-text-centered">
                                <tr v-for="type in types" :key="type.id" v-if="types.length">
                                    <td>
                                        {{ type.name }}&emsp;&emsp;
                                        <a class="mdi mdi-delete-forever has-text-current" @click="deleteSrieGenreOrType(type.id, 'type')"></a>
                                    </td>
                                </tr>
                                <p v-else class="has-text-centered">No data</p>
                            </tbody>
                        </table>
                        <table class="table is-fullwidth is-striped" v-show="showGenre">
                            <thead class="has-text-centered">
                                <tr>
                                    <th>
                                        <input id="serie-type-input" class="input editable is-small has-text-centered" type="text" @keyup.enter="addSerieGenre" v-model="newSerieGenre" placeholder="Type here to add a new Serie Genre"/>
                                    </th>
                                </tr>
                            </thead>
                            <tbody class="has-text-centered">
                                <tr v-for="genre in genres" :key="genre.id" v-if="genres.length">
                                    <td>
                                        {{ genre.name }}&emsp;&emsp;
                                        <a class="mdi mdi-delete-forever has-text-current" @click="deleteSrieGenreOrType(genre.id, 'genre')"></a>
                                    </td>
                                </tr>
                                <p v-else class="has-text-centered">No data</p>
                            </tbody>
                        </table>
                        <span class="column is-one-fifth-fullhd"></span>
                    </div>
                </div>
            </div>
            <footer class="card-footer has-text-centered">
                <a class="card-footer-item mdi mdi-arrow-left-circle has-text-current" @click="$router.go(-1)"> Back</a>
            </footer>
        </div>
    </div>
    <p v-else class="has-text-centered">No data</p>
</template>

<script setup lang="ts">
    import { onMounted, Ref, ref, watch } from 'vue'
    import SerieGenre from '@/types/serie-genre'
    import SerieType from '@/types/serie-type'
    import SerieSettingsService from '@/services/serie-settings-service'
    import SerieGenreRequest from '@/types/serie-genre-request'
    import SerieTypeRequest from '@/types/serie-type-request'

    const serieSettingsService = new SerieSettingsService()
    const genres: Ref<SerieGenre[]> = ref([])
    const types: Ref<SerieType[]> = ref([])
    const newSerieType = ref("")
    const newSerieGenre = ref("")

    const shouldRender = ref(false)
    const showType = ref(false)
    const showGenre = ref(false)

    onMounted(async () => {
        getGenres()
        getTypes()
        shouldRender.value = true
        showType.value = true
    })

    function showSection(section: string) {
        if (section === 'type') {
            showType.value = true
            showGenre.value = false
            document.getElementById('tab-type')?.classList.add('is-active')
            document.getElementById('tab-genre')?.classList.remove('is-active')
        } else {
            showType.value = false
            showGenre.value = true
            document.getElementById('tab-type')?.classList.remove('is-active')
            document.getElementById('tab-genre')?.classList.add('is-active')
        }
    }

    async function getGenres() {
        serieSettingsService.getSerieGenres().then(response => genres.value = response)
    }

    async function addSerieGenre() {
        const serieGenre = newSerieGenre.value
        if (serieGenre.length > 2) {
            var request: SerieGenreRequest = {
                name: serieGenre
            }
            newSerieGenre.value = ''
            serieSettingsService.addSerieGenre(request).then(() => getGenres())
        }
    }

    async function getTypes() {
        serieSettingsService.getSerieTypes().then(response => types.value = response)
    }

    async function addSerieType() {
        const serieType = newSerieType.value
        if (serieType.length > 2) {
            var request: SerieTypeRequest = {
                name: serieType
            }
            newSerieType.value = ''
            serieSettingsService.addSerieType(request).then(() => getTypes())
        }
    }

    async function deleteSrieGenreOrType(id: number, type: string) {
        if (type === 'genre') {
            serieSettingsService.deleteSerieGenre(id).then(() => getGenres())
        } else {
            serieSettingsService.deleteSerieType(id).then(() => getTypes())
        }
    }

    watch(genres, (newValues, oldValues) => {
        if (newValues.length != oldValues.length) {
            getGenres()
        }
    })
    watch(types, (newValues, oldValues) => {
        if (newValues.length != oldValues.length) {
            getTypes()
        }
    })
</script>