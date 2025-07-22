<template>
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
                    <p class="title is-5">
                        Name:&emsp;<input id="new-serie-name" class="input is-small" type="text" v-model.trim="serie.name" /><span></span>
                    </p>
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
                            <td><input class="input is-small" type="number" v-model="serie.season" /></td>
                            <td><input class="input is-small" type="number" v-model="serie.chapter" /></td>
                            <td><input class="input is-small" type="number" v-model="serie.score" /></td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        <footer class="card-footer">
            <a class="card-footer-item mdi mdi-arrow-left-circle has-text-current" @click="$router.go(-1)"> Back</a>
            <a id="add-button" class="card-footer-item mdi mdi-delete has-text-current" @click="addSerie"> Add</a>
            <span class="card-footer-item"></span>
        </footer>
    </div>
</template>

<script setup lang="ts">
    import { SerieService } from '@/services/'
    import SerieRequest from '@/types/serie-request'

    const serieService = new SerieService()
    var serie: SerieRequest = {
        name: '',
        season: 0,
        chapter: 0,
        score: 0
    }

    async function addSerie() {
        if (serie.name != '') {
            serieService.addSerie(serie)
        }
    }
</script>
