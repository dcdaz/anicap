import ky from "ky"
import { useRouter } from "vue-router";


class Api {
    private router = useRouter()

    public webApi = ky.create({
        prefixUrl: import.meta.env.VITE_BACKEND_URL,
        hooks: {
            afterResponse: [
                async (_request, _options, response) => {
                    switch (response.status) {
                        case 200:
                            break
                        case 201:
                        case 204:
                            if (response.url.includes('logout')) {
                                this.router.push('login')
                            }
                            break
                        case 401:
                            this.router.push({ name: 'login' })
                            break
                        case 404:
                            this.router.push({ name: 'not-found' })
                            break
                        case 502:
                            this.router.push({ name: 'bad-gateway' })
                            break
                    }
                    const headers = new Headers(_request.headers);
                    if (headers.get('returnToPreviousPage') == 'true') {
                        this.router.go(-1)
                    }
                    return
                },
            ],
        },
    })
}

export default Api;