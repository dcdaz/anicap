import { createApp } from 'vue'
import router from './router/router'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import App from './App.vue'

const app = createApp(App);

// define global properties
app.provide('baseUrl', 'http://127.0.0.1:8085')
app.config.globalProperties.baseUrl = 'http://127.0.0.1:8085'

const piniaStore = createPinia().use(({ store }) => {
  store.baseUrl = app.config.globalProperties.baseUrl
}).use(piniaPluginPersistedstate)

app
    .use(router)
    .use(piniaStore)
    .mount('#app')
