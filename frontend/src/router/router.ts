import { Login, AllSeriesDashboard, SerieDashboard, NewSerie, NotFound, BadGateway } from "@/components";
import { createRouter, createWebHistory } from "vue-router";

const routes = [
    {
        path: '/login',
        name: 'login',
        component: Login
    },
    {
        path: '/',
        name: 'home',
        component: AllSeriesDashboard
    },
    {
        path: '/serie-dashboard/:serieId',
        name: 'serie-dashboard',
        component: SerieDashboard
    },
    {
        path: '/add-serie',
        name: 'add-serie',
        component: NewSerie
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'not-found',
        component: NotFound
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'bad-gateway',
        component: BadGateway
    },
]

const router =  createRouter({
    history: createWebHistory(),
    routes: routes
})

router.beforeEach((to, from, next) => {
    next()
})

export default router