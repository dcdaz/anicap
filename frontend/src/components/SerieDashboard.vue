<template>
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
                        <input id="serie-title" class="input editable column is-one-third-fullhd is-size-5 title has-text-primary" type="text" v-model="serie.name" disabled/>
                    </div>
                    <a
                        :class="{ 'has-text-danger mdi mdi-heart': serie.favorite, 'has-text-current mdi mdi-heart-outline': !serie.favorite }"
                        @click="addToFavorite($event)"
                    ></a>
                    <a
                        :class="{ 'has-text-warning mdi mdi-star-box': serie.wishToSee, 'has-text-current mdi mdi-star-box-outline': !serie.wishToSee }"
                        @click="addtToWhishList($event)"
                    ></a>
                </div>

                <div class="content">
                    <table class="table is-fullwidth is-striped">
                        <thead class="has-text-centered">
                            <tr>
                                <th>Season</th>
                                <th>Chapter</th>
                                <th>Score</th>
                                <th>Watch Status</th>
                                <th>Details</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><input class="input editable is-small has-text-centered" type="number" v-model="serie.season" disabled /></td>
                                <td><input class="input editable is-small has-text-centered" type="number" v-model="serie.chapter" disabled /></td>
                                <td><input class="input editable is-small has-text-centered" type="number" v-model="serie.score" disabled /></td>
                                <td class="has-text-centered">
                                    <div class="select is-small">
                                        <select class="editable" v-model="serie.watchStatus" disabled>
                                            <option :value="0">{{ WatchStatus.NOT_WATCHING }}</option>
                                            <option :value="1">{{ WatchStatus.WATCHING }}</option>
                                            <option :value="2">{{ WatchStatus.WATCHED }}</option>
                                        </select>
                                    </div>
                                </td>
                                <td class="has-text-centered">
                                    <button class="button has-text-current is-small editable" @click="openModal('details-modal')" disabled>Open Details</button>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
            <footer class="card-footer has-text-centered">
                <a class="card-footer-item mdi mdi-arrow-left-circle has-text-current" @click="$router.go(-1)"> Back</a>
                <a class="card-footer-item has-text-current" @click="editSerie(true)">
                    <p v-show="shouldEdit" @click="updateSerie">
                        <span class="mdi mdi-content-save"></span> Save
                    </p>
                    <p v-show="!shouldEdit">
                        <span class="mdi mdi-note-edit"></span> Edit
                    </p>
                </a>
                <a class="card-footer-item mdi mdi-delete has-text-current" @click="deleteSerie"> Delete</a>
            </footer>
        </div>
    </div>
    <p v-else class="has-text-centered">No data</p>

    <!-- Serie Details Modal -->

     <div id="details-modal" class="modal has-text-centered">
        <div class="modal-background" @click="closeModal('details-modal')"></div>
        <div class="modal-content">
            <div class="box">
                <div class="tabs is-centered is-boxed">
                    <ul>
                        <li id="tab-type" class="is-active"><a @click="showSection('type')">Serie Types</a></li>
                        <li id="tab-genre"><a @click="showSection('genre')">Serie Genres</a></li>
                    </ul>
                    <button class="delete" aria-label="close" @click="closeModal('details-modal')"></button>
                </div>
                <div class="select is-multiple is-small" v-show="showType">
                    <select v-model="serie.serieTypeIds" multiple>
                        <option v-for="type in types" :value="type.id">{{ type.name }}</option>
                    </select>
                </div>
                <div class="select is-multiple is-small" v-show="showGenre">
                    <select v-model="serie.serieGenreIds" multiple>
                        <option v-for="genre in genres" :value="genre.id">{{ genre.name }}</option>
                    </select>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    import { onMounted, ref, Ref } from 'vue'
    import { SerieService } from '@/services/'
    import Serie from '@/types/serie'
    import WatchStatus from '@/types/status'
    import SerieGenre from '@/types/serie-genre'
    import SerieType from '@/types/serie-type'
    import SerieSettingsService from '@/services/serie-settings-service'

    const serieService = new SerieService()
    const serieSettingsService = new SerieSettingsService()
    const shouldRender = ref(false)
    const shouldEdit = ref(false)
    const serie: Ref<Serie> = ref<Serie>({
        id: 0,
        name: '',
        season: 0,
        chapter: 0,
        score: 0.0,
        favorite: false,
        wishToSee: false,
        serieTypeIds: [],
        serieGenreIds: [],
        watchStatus: 0,
    })
    const genres: Ref<SerieGenre[]> = ref([])
    const types: Ref<SerieType[]> = ref([])

    const showType = ref(false)
    const showGenre = ref(false)

    onMounted(async () => {
        serieService.get().then(response => {
            shouldRender.value = true
            serie.value = response
        })
        getTypes().then(() => getGenres())
    })

    function editSerie(isEditable: boolean) {
        const inputs = document.getElementsByClassName('editable')
        for (let i = 0; i < inputs.length; i++) {
            (inputs[i] as HTMLInputElement).disabled = !isEditable
        }
        shouldEdit.value = isEditable
    }

    function changeTitleAndElementColor(currentElement: Element | null, condition: boolean, className: string) {
        const titleElement = document.getElementById('serie-title')
        condition
            ? currentElement?.classList.replace('has-text-current', className)
            : currentElement?.classList.replace(className, 'has-text-current')
        condition ? titleElement?.classList.add(className) : titleElement?.classList.remove(className)
    }

    function openModal(id: string) {
        showType.value = true
        document.getElementById(id)?.classList.add('is-active')
    }

    function closeModal(id: string) {
        document.getElementById(id)?.classList.remove('is-active')
        document.getElementById('tab-type')?.classList.add('is-active')
        document.getElementById('tab-genre')?.classList.remove('is-active')
        showGenre.value = false
        console.log("Serie Types: " + serie.value.serieTypeIds?.join(', '))
        console.log("Serie Genres: " + serie.value.serieGenreIds?.join(', '))
    }

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

    async function updateSerie() {
        serieService.updateSerie(serie.value).then(() => editSerie(false))
    }

    async function addToFavorite(event: Event) {
        serie.value.favorite = !serie.value.favorite
        const currentElement = event?.target as Element
        currentElement.className = `has-text-current mdi ${serie.value.favorite ? 'mdi-heart' : 'mdi-heart-outline'}`
        changeTitleAndElementColor(currentElement, serie.value.favorite, 'has-text-danger')
        updateSerie()
    }

    async function addtToWhishList(event: Event) {
        serie.value.wishToSee = !serie.value.wishToSee
        const currentElement = event?.target as Element
        currentElement.className = `has-text-current mdi ${serie.value.wishToSee ? 'mdi-star-box' : 'mdi-star-box-outline'}`
        changeTitleAndElementColor(currentElement, serie.value.wishToSee, 'has-text-warning')
        updateSerie()
    }

    async function deleteSerie() {
        serieService.deleteSerie()
    }

    async function getTypes() {
        serieSettingsService.getSerieTypes().then(response => types.value = response)
    }

    async function getGenres() {
        serieSettingsService.getSerieGenres().then(response => genres.value = response)
    }
</script>
