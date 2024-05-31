import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import TodoView from '../views/TodoView.vue'
import AuthView from '../views/AuthView.vue'


const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/auth',
      name: 'auth',
      component: AuthView
    },
    {
      path: '/todos',
      name: 'todos',
      component: HomeView
    },
    {
      path: '/todos/:id',
      name: 'todo',
      component: TodoView
    }
  ]
})

export default router
