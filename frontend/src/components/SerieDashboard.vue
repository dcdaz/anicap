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
                        <p class="title is-4">{{ serie.name }}</p>
                    </div>
                </div>

                <div class="content">
                    <table class="table is-fullwidth is-striped">
                        <thead>
                            <tr>
                                <th>Season</th>
                                <th>Chapter</th>
                                <th>Score</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><input class="input editable is-small" type="number" v-model="serie.season" disabled /></td>
                                <td><input class="input editable is-small" type="number" v-model="serie.chapter" disabled /></td>
                                <td><input class="input editable is-small" type="number" v-model="serie.score" disabled /></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
            <footer class="card-footer">
                <a class="card-footer-item mdi mdi-arrow-left-circle has-text-current" @click="$router.go(-1)"> Back</a>
                <a class="card-footer-item has-text-current" @click="editSerie(true)">
                    <p v-if="shouldEdit" @click="updateSerie">
                        <span class="mdi mdi-content-save"></span> Save
                    </p>
                    <p v-else>
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
    import { onMounted, reactive, ref } from 'vue'
    import { SerieService } from '@/services/'
    import Serie from '@/types/serie'

    const serieService = new SerieService()
    const shouldRender = ref(false)
    const shouldEdit = ref(false)
    var serie: Serie

    onMounted(async () => {
        serieService.get().then((response) => {
            shouldRender.value = true
            serie = reactive(response as Serie)
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

    async function deleteSerie() {
        serieService.deleteSerie()
    }
</script>