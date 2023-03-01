<template>
  <form @submit.prevent="doRegister">
    <input v-model="name" type="text" placeholder="User name" required />
    <input v-model="password" type="password" placeholder="Password" required />
    <input value="Register" type="submit" />
  </form>
</template>

<script lang="ts">
import { login, register } from "@/api/api";
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
    doRegister: async function () {
      await register({ name: this.name, password: this.password });
      const token = await login({ name: this.name, password: this.password });
      this.$cookies.set("Authorization", token.Authorization);
      location.assign("rooms");
    },
  },
});
</script>
