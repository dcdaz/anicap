import { createApp } from 'vue'
import router from './router/router'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import App from './App.vue'

createApp(App)
    .use(router)
    .use(
      createPinia()
        .use(piniaPluginPersistedstate)
    )
    .mount('#app')
