import { defineStore } from "pinia"
import router from "@/router/router"
import Api from "@/utils/api"

const sessionStore = defineStore(
    'sessionStore',
    {
        state: () => {
            return {
                token: null
            }
        },
        persist: true,
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
                ).json()
                .then((response: any) => this.token = response.access_token)
                router.push({ name: 'home' })
            },
            async logout() {
                new Api().webApi.get(
                    'appuser/logout',
                    {
                        credentials: 'include'
                    }
                )
                .then(() => this.token = null)
                router.push({ name: 'login' })
            }
        }
    }
)

export default sessionStore
