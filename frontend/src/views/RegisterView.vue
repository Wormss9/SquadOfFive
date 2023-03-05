<template>
  <div>
    <form @submit.prevent="doRegister()">
      <input v-model="name" type="text" placeholder="User name" required />
      <input
        v-model="password"
        type="password"
        placeholder="Password"
        required
      />
      <input
        v-model="verifyPassword"
        type="password"
        placeholder="Verify Password"
        required
      />
      <input value="Register" type="submit" />
      <span class="error">{{ error }}</span>
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
      verifyPassword: "",
      error: "",
    };
  },
  methods: {
    doRegister: async function () {
      if (this.password != this.verifyPassword) {
        this.error = "Passwords don't match";
        return;
      }
      await register({ name: this.name, password: this.password });
      const token = await login({ name: this.name, password: this.password });
      this.$cookies.set("Authorization", token.Authorization);
      location.assign("rooms");
    },
  },
});
</script>
<style>
.error {
  color: red;
}
</style>
