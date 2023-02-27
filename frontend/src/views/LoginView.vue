<template>
  <form @submit.prevent="doLogin">
    <input v-model="name" type="text" placeholder="User name" required />
    <input v-model="password" type="password" placeholder="Password" required />
    <input type="submit" />
  </form>
</template>

<script lang="ts">
import { login } from "@/api/api";
import { defineComponent } from "vue";

export default defineComponent({
  // type inference enabled
  data() {
    return {
      name: "",
      password: "",
    };
  },
  methods: {
    doLogin: async function () {
      const token = await login({ name: this.name, password: this.password });
      this.$cookies.set("Authorization", token.Authorization);
      this.$router.push("rooms");
    },
  },
});
</script>
