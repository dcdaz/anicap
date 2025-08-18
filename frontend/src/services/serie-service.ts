import Serie from '@/types/serie'
import SerieRequest from '@/types/serie-request'
import Api from '@/utils/api'
import { useRoute } from 'vue-router'

class SerieService {

    private route = useRoute()
    private webApi = new Api().webApi

    async get(): Promise<Serie> {
        return await this.webApi.get(
            `serie/${this.route.params.serieId}`,
            {
                credentials: 'include'
            }
        ).json<Serie>()
    }

    async addSerie(request: Partial<SerieRequest>) {
        await this.webApi.post(
            'serie',
            {
                json: request,
                credentials: 'include'
            }
        )
    }

    async updateSerie(request: Partial<SerieRequest>, serieId: number | null = null) {
        const validSerieId = serieId != null ? serieId : this.route.params.serieId
        await this.webApi.put(
            `serie/${validSerieId}`,
            {
                json: request,
                credentials: 'include'
            }
        )
    }

    async deleteSerie() {
        await this.webApi.delete(
            `serie/${this.route.params.serieId}`,
            {
                credentials: 'include'
            }
        )
    }
}

export default SerieService