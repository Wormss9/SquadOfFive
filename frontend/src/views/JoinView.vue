<template>
  <div></div>
</template>

<script lang="ts">
import { join_room } from "@/api/api";
import { defineComponent } from "vue";
import { toast } from "vue3-toastify";

export default defineComponent({
  methods: {
    async join_room() {
      const room = this.$route.params.ulid;
      try {
        await join_room(room as string);
      } catch (error) {
        this.$router.push({ path: `/rooms` });
        toast.error("Room is full");
        return;
      }
      this.$router.push({ path: `/game/${room}` });
    },
  },
  async beforeMount() {
    this.join_room();
  },
});
</script>
