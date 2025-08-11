import { defineStore } from "pinia"
import ky from "ky"
import router from "@/router/router"

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
            getLoginUrl() {
                return `${process.env.VUE_APP_BACKEND_URL}/appuser`
            },
            async login(username: string, password: string) {
                const loginRequest = {
                    username: username,
                    password: password
                }
                
                await ky.post(
                    `${this.getLoginUrl()}/login`,
                    {
                        json: loginRequest,
                        credentials: 'include'
                    }
                ).json()
                .then((response: any) => this.token = response.access_token)
                .catch((error) => console.error("Ky error: ", error));
                router.push({ name: 'home' })
            },
            async logout() {
                ky.get(
                    `${this.getLoginUrl()}/logout`,
                    {
                        credentials: 'include'
                    }
                )
                .then(() => this.token = null)
                .catch((error) => console.error("Ky error: ", error));
                router.push({ name: 'login' })
            }
        }
    }
)

export default sessionStore
