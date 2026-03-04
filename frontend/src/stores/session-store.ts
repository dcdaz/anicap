import { defineStore } from "pinia"
import router from "@/router/router"
import Api from "@/utils/api"
import AppUserToken from "@/types/app-user-token"

const sessionStore = defineStore(
    'sessionStore',
    {
        state: () => ({
            accessToken: '',
        }),
        actions: {
            async login(username: string, password: string) {
                const loginRequest = {
                    username: username,
                    password: password
                }
                
                await new Api().webApi.post(
                    'appuser/login',
                    {
                        json: loginRequest,
                        credentials: 'include'
                    }
                ).json<AppUserToken>()
                .then(response => this.accessToken = response.accessToken)
                router.push({ name: 'home' })
            },
            async logout() {
                new Api().webApi.get(
                    'appuser/logout',
                    {
                        credentials: 'include'
                    }
                )
                .then(() => this.accessToken = '')
                router.push({ name: 'login' })
            }
        },
        persist: true,
    }
)

export default sessionStore
