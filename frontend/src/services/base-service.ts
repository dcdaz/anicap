import { useRoute, useRouter } from "vue-router"

class BaseService {

    protected baseUrl = process.env.VUE_APP_BACKEND_URL
    protected route = useRoute()
    protected router = useRouter()
}

export default BaseService