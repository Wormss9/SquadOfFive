<template>
  <div>
    <form @submit.prevent="doLogin()">
      <input v-model="name" type="text" placeholder="User name" required />
      <input
        v-model="password"
        type="password"
        placeholder="Password"
        required
      />
      <input value="Login" type="submit" />
    </form>
  </div>
</template>

<script lang="ts">
import { login } from "@/api/api";
import { AxiosError } from "axios";
import { defineComponent } from "vue";
import { toast } from "vue3-toastify";

export default defineComponent({
  data() {
    return {
      name: "",
      password: "",
    };
  },
  methods: {
    doLogin: async function () {
      try {
        const token = await login({ name: this.name, password: this.password });
        this.$cookies.set("Authorization", token.Authorization);
        location.assign("rooms");
      } catch (e) {
        const error = e as AxiosError;
        toast.error(error.response?.data as string);
      }
    },
  },
});
</script>
