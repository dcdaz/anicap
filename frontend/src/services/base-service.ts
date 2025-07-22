import { inject } from "vue"
import { useRoute, useRouter } from "vue-router"

class BaseService {

    protected baseUrl = inject('baseUrl')
    protected route = useRoute()
    protected router = useRouter()
}

export default BaseService