import ky from 'ky'
import Serie from '@/types/serie'
import BaseService from '@/services/base-service'

class SearchService extends BaseService {

    async searchSeries(): Promise<void | Serie[]> {
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
}

export default SearchService
