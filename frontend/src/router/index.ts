import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import TodoView from '../views/TodoView.vue'
import AuthView from '../views/AuthView.vue'
import SignUpView from '../views/SignUpView.vue'
import { any } from 'zod'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    
    {
      path: '/auth',
      name: 'auth',
      component: AuthView
    },
    {
      path: '/signup',
      name: 'signup',
      component: SignUpView
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

router.beforeEach(async (to, from, next) => {
  try {
    const response = await fetch('http://localhost:6969/auth/userinfo', {
      method: 'GET'
    });

    if (!response.ok&& to.name !== 'signup' && to.name !== 'auth'&& from.name !=='auth') {
      console.log(response.status);
      console.log("Response status is not OK, redirecting to /auth");
      return next({ path: '/auth' });
    }

  } catch (error) {
    console.error('Fetch error:', error);
    return next({ path: '/auth' });
  }

  if (!localStorage.getItem('TOKEN') && to.name !== 'auth'&& to.name !== 'signup') {
    console.log("No token found, redirecting to /auth");
    return next({ path: '/auth' });
  }

  next();
});

export default router
