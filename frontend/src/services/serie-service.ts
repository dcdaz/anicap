import ky from 'ky'
import Serie from '@/types/serie'
import BaseService from '@/services/base-service'

class SerieService extends BaseService {

    async get(): Promise<void | Serie> {
        return await ky.get(
            `${this.baseUrl}/serie/${this.route.params.serieId}`,
            {
                credentials: 'include'
            }
        ).json<Serie>()
        .catch((error) => {
            if (error.response.status === 401) {
                this.router.push({ name: 'login' })
            }
        })
    }

    async updateSerie(payload: any) {
        await ky.put(
            `${this.baseUrl}/serie/${this.route.params.serieId}`,
            {
                json: payload,
                credentials: 'include'
            }
        )
        .catch((error) => {
            if (error.response.status === 401) {
                this.router.push({ name: 'login' })
            }
        })
    }

    async deleteSerie() {
        await ky.delete(
            `${this.baseUrl}/serie/${this.route.params.serieId}`,
            {
                credentials: 'include'
            }
        )
        .then((response) => {
            if (response.status === 204) {
                this.router.push({ name: 'home'})
            }
        })
        .catch((error) => {
            if (error.response.status === 401) {
                this.router.push({ name: 'login' })
            }
        })
    }
}

export default SerieService