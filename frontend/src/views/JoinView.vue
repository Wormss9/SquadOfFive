<template>
  <div></div>
</template>

<script lang="ts">
import { join_room } from "@/api/api";
import { defineComponent } from "vue";

export default defineComponent({
  methods: {
    join_room() {
      const room = this.$route.params.ulid;
      join_room(room as string);
      location.assign(`/game/${room}`);
    },
  },
  async beforeMount() {
    this.join_room();
  },
});
</script>
