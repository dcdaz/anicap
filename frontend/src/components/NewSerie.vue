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
                        Name:&emsp;<input id="new-serie-name" class="input is-small" type="text" v-model.trim="serie.name" />
                    </p>
                </div>
            </div>

            <div class="content">
                <table class="table is-fullwidth is-striped">
                    <thead class="has-text-centered">
                        <tr>
                            <th>Season</th>
                            <th>Chapter</th>
                            <th>Score</th>
                            <th>Watch Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><input class="input is-small has-text-centered" type="number" v-model="serie.season" /></td>
                            <td><input class="input is-small has-text-centered" type="number" v-model="serie.chapter" /></td>
                            <td><input class="input is-small has-text-centered" type="number" v-model="serie.score" /></td>
                            <td class="has-text-centered">
                                <div class="select is-small">
                                    <select v-model="serie.watchStatus">
                                        <option :value="0">{{ WatchStatus.NOT_WATCHING }}</option>
                                        <option :value="1">{{ WatchStatus.WATCHING }}</option>
                                        <option :value="2">{{ WatchStatus.WATCHED }}</option>
                                    </select>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        <footer class="card-footer has-text-centered">
            <a class="card-footer-item mdi mdi-arrow-left-circle has-text-current" @click="$router.go(-1)"> Back</a>
            <a id="add-button" class="card-footer-item mdi mdi-delete has-text-current" @click="addSerie(false)"> Add</a>
            <a class="card-footer-item mdi mdi-star-box has-text-current" @click="addSerie(true)"> Add to wish list</a>
        </footer>
    </div>
</template>

<script setup lang="ts">
    import { SerieService } from '@/services/'
    import SerieRequest from '@/types/serie-request'
    import WatchStatus from '@/types/status'

    const serieService = new SerieService()
    var serie: SerieRequest = {
        name: '',
        season: 0,
        chapter: 0,
        score: 0,
        favorite: false,
        wishToSee: false,
        watchStatus: 0,
    }

    async function addSerie(wishToSee: boolean) {
        if (serie.name != '') {
            serie.wishToSee = wishToSee
            serieService.addSerie(serie)
        } else {
            document.getElementById('new-serie-name')?.focus()
        }
    }
</script>
