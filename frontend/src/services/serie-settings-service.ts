import SerieGenre from "@/types/serie-genre";
import SerieGenreRequest from "@/types/serie-genre-request";
import SerieType from "@/types/serie-type";
import SerieTypeRequest from "@/types/serie-type-request";
import Api from "@/utils/api";

class SerieSettingsService {

    private settingsPath = 'serie/settings'
    private webApi = new Api().webApi.extend({
        credentials: 'include'
    })

    async addSerieGenre(request: SerieGenreRequest) {
        await this.webApi.post(
            `${this.settingsPath}/genre`,
            {
                json: request
            }
        )
    }

    async getSerieGenres(): Promise<SerieGenre[]> {
        return await this.webApi.get(`${this.settingsPath}/genre`).json<SerieGenre[]>()
    }

    async deleteSerieGenre(serieGenreId: number) {
        await this.webApi.delete(`${this.settingsPath}/genre/${serieGenreId}`)
    }

    async addSerieType(request: SerieTypeRequest) {
        await this.webApi.post(
            `${this.settingsPath}/type`,
            {
                json: request
            }
        )
    }

    async getSerieTypes(): Promise<SerieType[]> {
        return await this.webApi.get(`${this.settingsPath}/type`).json<SerieType[]>()
    }

    async deleteSerieType(serieTypeId: number) {
        await this.webApi.delete(`${this.settingsPath}/type/${serieTypeId}`)
    }
}

export default SerieSettingsService
