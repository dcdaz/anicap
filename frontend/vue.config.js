module.exports = {
    transpileDependencies: true,
    publicPath: '/',
    pages: {
        index: {
            entry: 'src/main.ts',
            title: process.env.VUE_APP_TITLE,
        },
    },
}