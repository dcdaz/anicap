import Serie from '@/types/serie'
import SerieRequest from '@/types/serie-request'
import Api from '@/utils/api'

class DashboardService {

    private webApi = new Api().webApi.extend({
        credentials: 'include'
    })

    async searchAllSeries(): Promise<Serie[]> {
        return await this.webApi.get('serie').json<Serie[]>()
    }

    async searchSeries(queryParams: Partial<SerieRequest>): Promise<Serie[]> {
        return await this.webApi.get(
            'serie',
            {
                searchParams: queryParams,
            }
        ).json<Serie[]>()
    }

    private orderSeries(event: Event, series: Serie[]) {
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

    orderBy<T extends keyof Serie>(event: Event, series: Serie[], prop: T) {
        const sortedSeries = series.sort((s1, s2) => {
            if (s1[prop] > s2[prop]) {
                return 1
            }
            if (s1[prop] < s2[prop]) {
                return -1
            }
            return 0
        })
        this.orderSeries(event, sortedSeries)
    }
}

export default DashboardService
