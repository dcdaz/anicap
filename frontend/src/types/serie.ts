type Serie = {
    id: number,
    name: string,
    season: number,
    chapter: number,
    score: number,
    favorite: boolean,
    wishToSee: boolean,
    serieTypeIds: number[],
    serieGenreIds: number[],
    watchStatus: number,
}

export default Serie
