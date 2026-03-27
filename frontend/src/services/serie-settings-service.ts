import SerieGenre from "@/types/serie-genre";
import SerieGenreRequest from "@/types/serie-genre-request";
import Api from "@/utils/api";
import { useRoute } from "vue-router";

class SerieSettingsService {
    private route = useRoute()
    private webApi = new Api().webApi

    async addSerieGenre(request: SerieGenreRequest) {
        await this.webApi.post(
            'serie/settings/genre',
            {
                json: request,
                credentials: 'include'
            }
        )
    }

    async getSerieGenres(): Promise<SerieGenre[]> {
        return await this.webApi.get(
            'serie/settings/genre',
            {
                credentials: 'include'
            }
        ).json<SerieGenre[]>()
    }
}

export default SerieSettingsService
