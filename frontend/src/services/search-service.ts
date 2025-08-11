import ky from 'ky'
import Serie from '@/types/serie'
import BaseService from '@/services/base-service'
import SerieRequest from '@/types/serie-request'

class SearchService extends BaseService {

    async searchAllSeries(): Promise<void | Serie[]> {
        return await ky.get(
            `${this.baseUrl}/serie`,
            {
                credentials: 'include'
            }
        ).json<Serie[]>()
        .catch((error) => {
            if (error.response.status === 401) {
                this.router.push({ name: 'login' })
            }
        })
    }

    async searchSeries(queryParams: Partial<SerieRequest>): Promise<void | Serie[]> {
        return await ky.get(
            `${this.baseUrl}/serie`,
            {
                credentials: 'include',
                searchParams: queryParams,
            }
        ).json<Serie[]>()
        .catch((error) => {
            if (error.response.status === 401) {
                this.router.push({ name: 'login' })
            }
        })
    }
}

export default SearchService
