import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import cookies from "vue-cookies";

createApp(App).use(store).use(router).use(cookies).mount("#app");
