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
                        <!-- <p class="subtitle is-6">@johnsmith</p> -->
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
                                <td><input class="input editable" type="number" v-model="serie.season" disabled /></td>
                                <td><input class="input editable" type="number" v-model="serie.chapter" disabled /></td>
                                <td><input class="input editable" type="number" v-model="serie.score" disabled /></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
            <footer class="card-footer">
                <a class="card-footer-item mdi mdi-arrow-left-circle has-text-current" @click="$router.go(-1)"> Back</a>
                <a class="card-footer-item has-text-current" @click="edit(true)">
                    <p v-if="shouldEdit" @click="update">
                        <span class="mdi mdi-content-save"></span> Save
                    </p>
                    <p v-else>
                        <span class="mdi mdi-note-edit"></span> Edit
                    </p>
                </a>
                <a class="card-footer-item mdi mdi-delete has-text-current"> Delete</a>
            </footer>
        </div>
    </div>
    <p v-else class="has-text-centered">No data</p>
</template>

<script setup>
    import ky from 'ky';
    import { useRoute, useRouter } from 'vue-router';
    import { inject, onMounted, reactive, ref } from 'vue';

    const baseUrl = inject('baseUrl')
    const route = useRoute()
    const router = useRouter()
    const shouldRender = ref(false)
    const shouldEdit = ref(false)
    var serie = {}

    onMounted(async () => {
        await ky.get(
            `${baseUrl}/serie/${route.params.serieId}`,
            {
                credentials: 'include'
            }
        ).json()
        .then((data) => {
            shouldRender.value = true
            serie = reactive({
                name: data.name,
                season: data.season,
                chapter: data.chapter,
                score: data.score
            })
        })
        .catch((error) => {
            if (error.response.status === 401) {
                router.push({ name: 'login' })
            }
        })
    })

    function edit(isEditable) {
        const inputs = document.getElementsByClassName('editable')
        for (let i = 0; i < inputs.length; i++) {
            inputs[i].disabled = !isEditable
        }
        shouldEdit.value = isEditable
    }

    async function update() {
        const payload = {
            name: serie.name,
            season: serie.season,
            chapter: serie.chapter,
            score: serie.score
        }
        await ky.put(
            `${baseUrl}/serie/${route.params.serieId}`,
            {
                json: payload,
                credentials: 'include'
            }
        )
        .catch((error) => {
            if (error.response.status === 401) {
                router.push({ name: 'login' })
            }
        })
        edit(false)
    }
</script>