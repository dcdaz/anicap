import Serie from '@/types/serie'
import SerieRequest from '@/types/serie-request'
import Api from '@/utils/api'

class DashboardService {

    private webApi = new Api().webApi

    async searchAllSeries(): Promise<void | Serie[]> {
        return await this.webApi.get(
            'serie',
            {
                credentials: 'include'
            }
        ).json<Serie[]>()
    }

    async searchSeries(queryParams: Partial<SerieRequest>): Promise<void | Serie[]> {
        return await this.webApi.get(
            'serie',
            {
                credentials: 'include',
                searchParams: queryParams,
            }
        ).json<Serie[]>()
    }

    orderSeries(event: Event, series: Serie[]) {
        const currentElement = (event.currentTarget as Element)
        const currentClass = (currentElement.attributes as NamedNodeMap)[0].value
        if (currentClass.includes('mdi-menu-swap')) {
            currentElement.classList.remove('mdi-menu-swap')
            currentElement.classList.add('mdi-menu-up')
        } else if (currentClass.includes('mdi-menu-up')) {
            currentElement.classList.remove('mdi-menu-up')
            currentElement.classList.add('mdi-menu-down')
            series.reverse()
        } else {
            currentElement.classList.remove('mdi-menu-down')
            currentElement.classList.add('mdi-menu-swap')
            series.sort((s1, s2) => {
                if (s1.id > s2.id) {
                    return 1
                }
                if (s1.id < s2.id) {
                    return -1
                }
                return 0
            })
        }
    }

    orderByName(event: Event, series: Serie[]) {
        const sortedSeries = series.sort((s1, s2) => {
            if (s1.name > s2.name) {
                return 1
            }
            if (s1.name < s2.name) {
                return -1
            }
            return 0
        })
        this.orderSeries(event, sortedSeries)
    }

    orderBySeason(event: Event, series: Serie[]) {
        const sortedSeries = series.sort((s1, s2) => {
            if (s1.season > s2.season) {
                return 1
            }
            if (s1.season < s2.season) {
                return -1
            }
            return 0
        })
        this.orderSeries(event, sortedSeries)
    }

    orderByChapter(event: Event, series: Serie[]) {
        const sortedSeries = series.sort((s1, s2) => {
            if (s1.chapter > s2.chapter) {
                return 1
            }
            if (s1.chapter < s2.chapter) {
                return -1
            }
            return 0
        })
        this.orderSeries(event, sortedSeries)
    }

    orderByScore(event: Event, series: Serie[]) {
        const sortedSeries = series.sort((s1, s2) => {
            if (s1.score > s2.score) {
                return 1
            }
            if (s1.score < s2.score) {
                return -1
            }
            return 0
        })
        this.orderSeries(event, sortedSeries)
    }

    orderByFavorite(event: Event, series: Serie[]) {
        const sortedSeries = series.sort((s1, s2) => {
            if (s1.favorite > s2.favorite) {
                return 1
            }
            if (s1.favorite < s2.favorite) {
                return -1
            }
            return 0
        })
        this.orderSeries(event, sortedSeries)
    }

    orderByWishList(event: Event, series: Serie[]) {
        const sortedSeries = series.sort((s1, s2) => {
            if (s1.wish_to_see > s2.wish_to_see) {
                return 1
            }
            if (s1.wish_to_see < s2.wish_to_see) {
                return -1
            }
            return 0
        })
        this.orderSeries(event, sortedSeries)
    }
}

export default DashboardService
