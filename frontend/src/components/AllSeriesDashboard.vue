<template>
    <div v-if="shouldRender">
        <div class="columns is-vcentered">
            <span class="column is-centered is-1">Search:</span>
            <input id="search-serie-input" class="input column is-one-third-fullhd is-one-quarter is-small" type="text" v-model="serieNameParam" @keyup="searchSeries" placeholder="Type here to search"/>
            <span class="column is-two-fifths-fullhd is-one-third is-offset-1"></span>
            <router-link :to="'/add-serie'" class="column mdi mdi-plus-box has-text-current"> Add serie</router-link>
        </div>
        <div class="box">
            <table class="table is-fullwidth is-striped">
                <thead>
                    <tr>
                        <th>Name<button class="mdi mdi-menu-swap" @click="dashboardService.orderBy($event, series, 'name')"/></th>
                        <th class="has-text-centered">Season<button class="mdi mdi-menu-swap" @click="dashboardService.orderBy($event, series, 'season')"/></th>
                        <th class="has-text-centered">Chapter<button class="mdi mdi-menu-swap" @click="dashboardService.orderBy($event, series, 'chapter')"/></th>
                        <th class="has-text-centered">Score<button class="mdi mdi-menu-swap" @click="dashboardService.orderBy($event, series, 'score')"/></th>
                        <th class="has-text-centered">Watch Status<button class="mdi mdi-menu-swap" @click="dashboardService.orderBy($event, series, 'watchStatus')"/></th>
                        <th class="has-text-centered">Favorite<button class="mdi mdi-menu-swap" @click="dashboardService.orderBy($event, series, 'favorite')"/></th>
                        <th class="has-text-centered">Whish to see<button class="mdi mdi-menu-swap" @click="dashboardService.orderBy($event, series, 'wishToSee')"/></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="serie in series" :key="serie.id">
                        <td>
                            <router-link :to="`/serie-dashboard/${serie.id}`" class="has-text-current">{{ serie.name }}</router-link>
                        </td>
                        <td class="has-text-centered">
                            {{ serie.season }}
                        </td>
                        <td class="has-text-centered">
                            {{ serie.chapter }}
                        </td>
                        <td class="has-text-centered">
                            {{ serie.score }}
                        </td>
                        <td
                            class="has-text-centered"
                            :class="{ 'has-text-info': serie.watchStatus === 1, 'has-text-success': serie.watchStatus === 2 }"
                        >
                            {{ Object.values(WatchStatus)[serie.watchStatus] }}
                        </td>
                        <td class="has-text-centered" :class="{ 'has-text-danger': serie.favorite }">
                            <a
                                :class="{ 'has-text-danger mdi mdi-heart': serie.favorite, 'has-text-current mdi mdi-heart-outline': !serie.favorite }"
                                @click="addToFavorite($event, serie)"
                            ></a>
                        </td>
                        <td class="has-text-centered" :class="{ 'has-text-warning': serie.wishToSee }">
                            <a
                                :class="{ 'has-text-warning mdi mdi-star-box': serie.wishToSee, 'has-text-current mdi mdi-star-box-outline': !serie.wishToSee }"
                                @click="addToWhishList($event, serie)"
                            ></a>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
    <p v-else class="has-text-centered">No data</p>
</template>

<script setup lang="ts">
    import { onMounted, Ref, ref } from 'vue'
    import Serie from '@/types/serie'
    import SerieRequest from '@/types/serie-request'
    import WatchStatus from '@/types/status'
    import { DashboardService, SerieService } from '@/services/'

    const dashboardService = new DashboardService()
    const serieService = new SerieService()
    const shouldRender = ref(false)
    const serieNameParam = ref("")
    const series: Ref<Serie[]> = ref([])

    onMounted(async () => {
        dashboardService.searchAllSeries().then(response => {
            shouldRender.value = true
            series.value = response
        })
        window.addEventListener('keypress', (event) => {
            if (event.key != '/') {
                return
            }
            const searchBox = document.getElementById('searchBox')
            if (document.activeElement === searchBox) {
                return
            }
            event.preventDefault()
            document.getElementById('search-serie-input')?.focus()
        })
    })

    async function searchSeries() {
        const queryParams: Partial<SerieRequest> = {}
        const name = serieNameParam.value
        if (name.length > 2) {
            queryParams.name = name
            dashboardService.searchSeries(queryParams).then(response => series.value = response)
        } else {
            dashboardService.searchAllSeries().then(response => series.value = response)
        }
    }

    async function addToFavorite(event: Event, serie: Serie) {
        serie.favorite = !serie.favorite
        const currentElement = event?.target as Element
        currentElement.className = `has-text-current mdi ${serie.favorite ? 'mdi-heart' : 'mdi-heart-outline'}`
        changeElementColor(currentElement, serie.favorite, 'has-text-danger')
        serieService.updateSerie(serie, serie.id)
    }

    async function addToWhishList(event: Event, serie: Serie) {
        serie.wishToSee = !serie.wishToSee
        const currentElement = event?.target as Element
        currentElement.className = `has-text-current mdi ${serie.wishToSee ? 'mdi-star-box' : 'mdi-star-box-outline'}`;
        changeElementColor(currentElement, serie.wishToSee, 'has-text-warning')
        serieService.updateSerie(serie, serie.id)
    }

    function changeElementColor(currentElement: Element | null, condition: boolean, className: string) {
        condition ? currentElement?.classList.add(className) : currentElement?.classList.remove(className)
    }
</script>
