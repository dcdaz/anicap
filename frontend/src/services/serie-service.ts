import Serie from '@/types/serie'
import SerieRequest from '@/types/serie-request'
import Api from '@/utils/api'
import { useRoute } from 'vue-router'

class SerieService {

    private route = useRoute()
    private webApi = new Api().webApi.extend({
        credentials: 'include'
    })

    async get(): Promise<Serie> {
        return await this.webApi.get(`serie/${this.route.params.serieId}`).json<Serie>()
    }

    async addSerie(request: SerieRequest) {
        await this.webApi.post(
            'serie',
            {
                json: request,
                headers: {
                    returnToPreviousPage: 'true'
                }
            }
        )
    }

    async updateSerie(request: SerieRequest, serieId: number | null = null) {
        const validSerieId = serieId != null ? serieId : this.route.params.serieId
        await this.webApi.put(
            `serie/${validSerieId}`,
            {
                json: request
            }
        )
    }

    async deleteSerie() {
        await this.webApi.delete(
            `serie/${this.route.params.serieId}`,
            {
                headers: {
                    returnToPreviousPage: 'true'
                }
            }
        )
    }
}

export default SerieService