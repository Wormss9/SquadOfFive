<template>
  <NavBar :loggedIn="loggedIn" />
  <router-view class="main" />
</template>
<script lang="ts">
import { defineComponent } from "vue";
import NavBar from "./components/NavBar.vue";
import "./style/style.scss";
import "vue3-toastify/dist/index.css";
export default defineComponent({
  components: {
    NavBar,
  },
  computed: {
    loggedIn() {
      return this.$cookies.get("Authorization") != null;
    },
  },
});
</script>
<style lang="scss">
.main {
  margin-top: 50px;
  height: calc(100vh - 50px);
}
</style>
