//引入路由对象
import { createRouter, createWebHistory, createWebHashHistory, createMemoryHistory, RouteRecordRaw } from 'vue-router'

//vue2 mode history vue3 createWebHistory
//vue2 mode  hash  vue3  createWebHashHistory
//vue2 mode abstact vue3  createMemoryHistory

//路由数组的类型 RouteRecordRaw
// 定义一些路由
// 每个路由都需要映射到一个组件。
const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: "Index",
        component: () => import('./components/Home.vue')
    },
    {
        path: '/student-list',
        name: "StudentList",
        component: () => import('./components/StudentList.vue')
    },
    {
        path: '/course-list',
        name: "CourseList",
        component: () => import('./components/CourseList.vue')
    },
    {
        path: '/update',
        name: "Update",
        component: () => import('./components/Update.vue'),
        children: [
            {
                path: "",
                name: "UpdateHome",
                component: () => import('./components/update/UpdateHome.vue')
            },
            {
                path: "student",
                name: "UpdateStudent",
                component: () => import('./components/update/UpdateStudent.vue')
            },
            {
                path: "course",
                name: "UpdateCourse",
                component: () => import('./components/update/UpdateCourse.vue')
            }
        ]
    }
]



const router = createRouter({
    history: createWebHistory(),
    routes
})

//导出router
export default router