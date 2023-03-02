<template>
  <nav>
    <LoggedInBar v-if="loggedIn" />
    <div v-else>
      <router-link to="/login">Login</router-link>
      <router-link to="/register">Register</router-link>
    </div>
  </nav>
  <router-view class="main" />
</template>
<script lang="ts">
import { defineComponent } from "vue";
import LoggedInBar from "./components/LoggedInBar.vue";
import "./style/style.scss";
import "vue3-toastify/dist/index.css";
export default defineComponent({
  components: {
    LoggedInBar,
  },
  computed: {
    loggedIn() {
      return this.$cookies.get("Authorization") != null;
    },
  },
});
</script>
