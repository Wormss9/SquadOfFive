<template>
  <router-link to="/rooms">Rooms</router-link>
  <router-link class="profile" to="/user">
    <div>{{ user.nick }}</div>
    <img
      v-if="user.avatar"
      v-bind:src="'data:image/png;base64,' + user.avatar"
      class="avatar"
    />
  </router-link>
</template>

<script lang="ts">
import { get_user } from "@/api/api";
import { defineComponent } from "vue";
import { User } from "../api/types";

export default defineComponent({
  data() {
    return {
      user: {} as User,
    };
  },
  methods: {
    getUser: async function () {
      try {
        this.user = await get_user();
      } catch (_) {
        this.$cookies.remove("Authorization");
        location.assign("");
      }
    },
  },
  beforeMount() {
    this.getUser();
  },
});
</script>
