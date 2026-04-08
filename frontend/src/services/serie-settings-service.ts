import SerieGenre from "@/types/serie-genre";
import SerieGenreRequest from "@/types/serie-genre-request";
import SerieType from "@/types/serie-type";
import SerieTypeRequest from "@/types/serie-type-request";
import Api from "@/utils/api";

class SerieSettingsService {
    private webApi = new Api().webApi
    private settingsPath = 'serie/settings'

    async addSerieGenre(request: SerieGenreRequest) {
        await this.webApi.post(
            `${this.settingsPath}/genre`,
            {
                json: request,
                credentials: 'include'
            }
        )
    }

    async getSerieGenres(): Promise<SerieGenre[]> {
        return await this.webApi.get(
            `${this.settingsPath}/genre`,
            {
                credentials: 'include'
            }
        ).json<SerieGenre[]>()
    }

    async deleteSerieGenre(serieGenreId: number) {
        await this.webApi.delete(
            `${this.settingsPath}/genre/${serieGenreId}`,
            {
                credentials: 'include'
            }
        )
    }

    async addSerieType(request: SerieTypeRequest) {
        await this.webApi.post(
            `${this.settingsPath}/type`,
            {
                json: request,
                credentials: 'include'
            }
        )
    }

    async getSerieTypes(): Promise<SerieType[]> {
        return await this.webApi.get(
            `${this.settingsPath}/type`,
            {
                credentials: 'include'
            }
        ).json<SerieType[]>()
    }

    async deleteSerieType(serieTypeId: number) {
        await this.webApi.delete(
            `${this.settingsPath}/type/${serieTypeId}`,
            {
                credentials: 'include'
            }
        )
    }
}

export default SerieSettingsService
