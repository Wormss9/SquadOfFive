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
      const x = this.$cookies.remove("Authorization");
      console.log(x);
      location.assign("login");
    },
  },
});
</script>
