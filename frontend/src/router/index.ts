import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import HomeView from "../views/HomeView.vue";
import RoomsView from "../views/RoomsView.vue";
import SigningView from "../views/SigningView.vue";
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
    component: SigningView,
    props: { login: true },
  },
  {
    path: "/register",
    component: SigningView,
    props: { login: false },
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
