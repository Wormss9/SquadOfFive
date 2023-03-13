<template>
  <nav v-if="loggedIn">
    <router-link to="/rooms">Rooms</router-link>
    <router-link class="profile right" to="/user">
      <div>{{ user.nick }}</div>
      <img v-bind:src="'data:image/png;base64,' + user.avatar" class="avatar" />
    </router-link>
  </nav>
  <nav v-else>
    <router-link class="right" to="/login">Login</router-link>
    <router-link to="/register">Register</router-link>
  </nav>
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
  props: {
    loggedIn: Boolean,
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
    if (this.loggedIn) {
      this.getUser();
    }
  },
});
</script>
<style lang="scss">
@import "../style/variables.scss";
nav {
  display: flex;
  background-color: $light-grey;
  position: fixed;
  top: 0;
  height: 3em;
  width: 100%;

  a {
    float: left;
    color: $white;
    padding: 1em;
    text-decoration: none;
    &:hover {
      background-color: $white;
      color: $black;
    }
    &.right {
      margin-left: auto;
    }
    &.profile {
      display: flex;
      padding: 0;
      * {
        padding: 0 1em;
        align-self: center;
      }
    }
  }
}
</style>
