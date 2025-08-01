import { Login, AllSeriesDashboard, SerieDashboard, NewSerie, NotFound } from "@/components";
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
        name: 'NotFound',
        component: NotFound
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