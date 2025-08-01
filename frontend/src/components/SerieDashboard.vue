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
                        <p
                            id="serie-title"
                            class="title is-4"
                            :class="{ 'has-text-primary': serie.wish_to_see, 'has-text-warning': serie.favorite }"
                        >
                            {{ serie.name }}
                        </p>
                    </div>
                    <a
                        :class="{ 'has-text-warning mdi mdi-heart': serie.favorite, 'has-text-current mdi mdi-heart-outline': !serie.favorite }"
                        @click="addToFavorite($event, serie)"
                    ></a>
                    <a
                        :class="{ 'has-text-primary mdi mdi-star-box': serie.wish_to_see, 'has-text-current mdi mdi-star-box-outline': !serie.wish_to_see }"
                        @click="addtToWhishList($event, serie)"
                    ></a>
                </div>

                <div class="content">
                    <table class="table is-fullwidth is-striped">
                        <thead class="has-text-centered">
                            <tr>
                                <th>Season</th>
                                <th>Chapter</th>
                                <th>Score</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><input class="input editable is-small has-text-centered" type="number" v-model="serie.season" disabled /></td>
                                <td><input class="input editable is-small has-text-centered" type="number" v-model="serie.chapter" disabled /></td>
                                <td><input class="input editable is-small has-text-centered" type="number" v-model="serie.score" disabled /></td>
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
</template>

<script setup lang="ts">
    import { onMounted, ref } from 'vue'
    import { SerieService } from '@/services/'
    import Serie from '@/types/serie'

    const serieService = new SerieService()
    const shouldRender = ref(false)
    const shouldEdit = ref(false)
    var serie: Serie

    onMounted(async () => {
        serieService.get().then((response) => {
            shouldRender.value = true
            serie = response as Serie
        })
    })

    function editSerie(isEditable: boolean) {
        const inputs = document.getElementsByClassName('editable')
        for (let i = 0; i < inputs.length; i++) {
            (inputs[i] as HTMLInputElement).disabled = !isEditable
        }
        shouldEdit.value = isEditable
    }

    async function updateSerie() {
        serieService.updateSerie(serie).then(() => editSerie(false))
    }

    function changeTitleAndElementColor(currentElement: Element | null, condition: boolean, className: string) {
        const titleElement = document.getElementById('serie-title')
        condition
            ? currentElement?.classList.replace('has-text-current', className)
            : currentElement?.classList.replace(className, 'has-text-current')
        condition ? titleElement?.classList.add(className) : titleElement?.classList.remove(className)
    }

    async function addToFavorite(event: Event, serie: Serie) {
        serie.favorite = !serie.favorite
        const currentElement = event?.target as Element
        currentElement.className = `has-text-current mdi ${serie.favorite ? 'mdi-heart' : 'mdi-heart-outline'}`
        changeTitleAndElementColor(currentElement, serie.favorite, 'has-text-warning')
        updateSerie()
    }

    async function addtToWhishList(event: Event, serie: Serie) {
        serie.wish_to_see = !serie.wish_to_see
        const currentElement = event?.target as Element
        currentElement.className = `has-text-current mdi ${serie.wish_to_see ? 'mdi-star-box' : 'mdi-star-box-outline'}`
        changeTitleAndElementColor(currentElement, serie.wish_to_see, 'has-text-primary')
        updateSerie()
    }

    async function deleteSerie() {
        serieService.deleteSerie()
    }
</script>