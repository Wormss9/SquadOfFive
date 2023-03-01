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
      this.user = await get_user();
    },
  },
  beforeMount() {
    this.getUser();
  },
});
</script>
