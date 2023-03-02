<template>
  <div>
    <form @submit.prevent="login ? doLogin() : doRegister()">
      <input v-model="name" type="text" placeholder="User name" required />
      <input
        v-model="password"
        type="password"
        placeholder="Password"
        required
      />
      <input :value="login ? 'Login' : 'Register'" type="submit" />
    </form>
  </div>
</template>

<script lang="ts">
import { login, register } from "@/api/api";
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    login: Boolean,
  },
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
      location.assign("rooms");
    },
    doRegister: async function () {
      await register({ name: this.name, password: this.password });
      const token = await login({ name: this.name, password: this.password });
      this.$cookies.set("Authorization", token.Authorization);
      location.assign("rooms");
    },
  },
});
</script>
