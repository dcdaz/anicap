import ky from "ky"
import { useRouter } from "vue-router";


class Api {
    private router = useRouter()

    public webApi = ky.create({
        prefixUrl: process.env.VUE_APP_BACKEND_URL,
        hooks: {
            afterResponse: [
                async (_request, _options, response) => {
                    switch (response.status) {
                        case 200:
                            return
                        case 201:
                        case 204:
                            this.router.push({ name: 'home'})
                            break
                        case 401:
                            this.router.push({ name: 'login' })
                            break
                        case 404:
                            this.router.push({ name: 'not-found' })
                            break
                        case 502:
                            this.router.push({name: 'bad-gateway'})
                            break
                    }
                    return new Response(null)
                },
            ],
        },
    })
}

export default Api;