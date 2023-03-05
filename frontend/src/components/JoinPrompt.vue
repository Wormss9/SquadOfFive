<template>
  <form @submit.prevent="join()">
    <input v-model="room" placeholder="Ulid" required />
    <input value="Join" type="submit" />
  </form>
</template>

<script lang="ts">
import { join_room } from "@/api/api";
import { defineComponent } from "vue";

export default defineComponent({
  methods: {
    join() {
      join_room(this.room);
      location.assign(`game/${this.room}`);
    },
  },
  data() {
    return {
      room: "",
    };
  },
});
</script>
