import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import cookies from "vue-cookies";
import Vue3Toastify, { toast, ToastContainerOptions } from "vue3-toastify";

createApp(App)
  .use(store)
  .use(router)
  .use(cookies)
  .use(Vue3Toastify, {
    autoClose: 3000,
    position: toast.POSITION.TOP_CENTER,
    className: "foo-bar",
    theme: "dark",
  } as ToastContainerOptions)
  .mount("#app");
