<template>
  <div>
    <button v-on:click="logout">Logout</button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  methods: {
    logout() {
      this.$cookies.remove("Authorization");
      location.assign("login");
    },
  },
});
</script>
