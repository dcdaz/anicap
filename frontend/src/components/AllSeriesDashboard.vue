<template>
    <div v-show="shouldRender">
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
                        <th>Name<button class="mdi mdi-menu-swap" @click="dashboardService.orderByName($event, series)"/></th>
                        <th class="has-text-centered">Season<button class="mdi mdi-menu-swap" @click="dashboardService.orderBySeason($event, series)"/></th>
                        <th class="has-text-centered">Chapter<button class="mdi mdi-menu-swap" @click="dashboardService.orderByChapter($event, series)"/></th>
                        <th class="has-text-centered">Score<button class="mdi mdi-menu-swap" @click="dashboardService.orderByScore($event, series)"/></th>
                        <th class="has-text-centered">Favorite<button class="mdi mdi-menu-swap" @click="dashboardService.orderByFavorite($event, series)"/></th>
                        <th class="has-text-centered">Whish to see<button class="mdi mdi-menu-swap" @click="dashboardService.orderByWishList($event, series)"/></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="serie in series" :key="serie.id">
                        <td :class="{ 'has-text-primary': serie.wish_to_see, 'has-text-warning': serie.favorite }">
                            <router-link :to="`/serie-dashboard/${serie.id}`" class="has-text-current">{{ serie.name }}</router-link>
                        </td>
                        <td
                            class="has-text-centered"
                            :class="{ 'has-text-primary': serie.wish_to_see, 'has-text-warning': serie.favorite }"
                        >
                            {{ serie.season }}
                        </td>
                        <td
                            class="has-text-centered"
                            :class="{ 'has-text-primary': serie.wish_to_see, 'has-text-warning': serie.favorite }"
                        >
                            {{ serie.chapter }}
                        </td>
                        <td
                            class="has-text-centered"
                            :class="{ 'has-text-primary': serie.wish_to_see, 'has-text-warning': serie.favorite }"
                        >
                            {{ serie.score }}
                        </td>
                        <td class="has-text-centered" :class="{ 'has-text-primary': serie.wish_to_see, 'has-text-warning': serie.favorite }">
                            <a
                                :class="{ 'has-text-warning mdi mdi-heart': serie.favorite, 'has-text-current mdi mdi-heart-outline': !serie.favorite }"
                                @click="addToFavorite($event, serie)"
                            ></a>
                        </td>
                        <td class="has-text-centered" :class="{ 'has-text-primary': serie.wish_to_see, 'has-text-warning': serie.favorite }">
                            <a
                                :class="{ 'has-text-primary mdi mdi-star-box': serie.wish_to_see, 'has-text-current mdi mdi-star-box-outline': !serie.wish_to_see }"
                                @click="addtToWhishList($event, serie)"
                            ></a>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>

<script setup lang="ts">
    import { onMounted, Ref, ref } from 'vue'
    import Serie from '@/types/serie'
    import SerieRequest from '@/types/serie-request'
    import { DashboardService, SerieService } from '@/services/'

    const dashboardService = new DashboardService()
    const serieService = new SerieService()
    const shouldRender = ref(false)
    const serieNameParam = ref("")
    const series: Ref<Serie[]> = ref([])

    onMounted(async () => {
        dashboardService.searchAllSeries().then((response) => {
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
            dashboardService.searchSeries(queryParams).then((response) => series.value = response)
        } else {
            dashboardService.searchAllSeries().then((response) => series.value = response)
        }
    }

    async function addToFavorite(event: Event, serie: Serie) {
        serie.favorite = !serie.favorite
        const currentElement = event?.target as Element
        currentElement.className = `has-text-current mdi ${serie.favorite ? 'mdi-heart' : 'mdi-heart-outline'}`
        changeRowColor(currentElement, serie.favorite, 'has-text-warning')
        serieService.updateSerie(serie, serie.id)
    }

    async function addtToWhishList(event: Event, serie: Serie) {
        serie.wish_to_see = !serie.wish_to_see
        const currentElement = event?.target as Element
        currentElement.className = `has-text-current mdi ${serie.wish_to_see ? 'mdi-star-box' : 'mdi-star-box-outline'}`;
        changeRowColor(currentElement, serie.wish_to_see, 'has-text-primary')
        serieService.updateSerie(serie, serie.id)
    }

    function changeRowColor(currentElement: Element | null, condition: boolean, className: string) {
        const wholeRow = (currentElement?.parentElement?.parentElement as Element).children
        for (let i = 0; i < wholeRow.length; i++) {
            condition ? wholeRow[i].classList.add(className) : wholeRow[i].classList.remove(className)
        }
    }
</script>
