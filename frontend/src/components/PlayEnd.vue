<template>
  <div class="gameEnd">
    <div class="players">
      <div v-for="player in players" :key="player.id">
        <p>{{ player.nick }}</p>
        <img
          v-bind:src="'data:image/png;base64,' + player.avatar"
          class="avatar"
        />
      </div>
    </div>
    <button @click="next" class="link-like-button button">Continue</button>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { Gamer } from "@/api/types";
export default defineComponent({
  props: {
    players: {
      type: Array as PropType<
        (Gamer & { userId: number; cards?: number; online?: boolean })[]
      >,
      default: () => [],
    },
  },
  methods: {
    next() {
      this.$emit("continue");
    },
  },
  computed: {},
});
</script>
<style lang="scss">
.gameEnd {
  position: absolute;
  top: 10%;
  left: 10%;
  width: 80%;
  height: 80%;
  border-radius: 1em;
  box-shadow: black 0px 16px 16px;
  z-index: 1000;
  background-color: #222;
}

.players {
  display: flex;
  justify-content: center;
  text-align: center;
}
</style>
