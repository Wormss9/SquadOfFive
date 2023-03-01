import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import HomeView from "../views/HomeView.vue";
import RoomsView from "../views/RoomsView.vue";
import LoginView from "../views/LoginView.vue";
import RegisterView from "../views/RegisterView.vue";
import GameView from "../views/GameView.vue";
import UserView from "../views/UserView.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    component: HomeView,
  },
  {
    path: "/rooms",
    component: RoomsView,
  },
  {
    path: "/login",
    component: LoginView,
  },
  {
    path: "/register",
    component: RegisterView,
  },
  {
    path: "/game/:ulid",
    component: GameView,
  },
  {
    path: "/user",
    component: UserView,
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
