<template>
  <div class="gameEnd">
    <div class="player" v-for="player in orderedPlayers" :key="player.nick">
      <div>{{ player.nick }}</div>
      <img
        v-bind:src="'data:image/png;base64,' + player.avatar"
        class="avatar"
      />
      <div>{{ player.score }}</div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { Gamer } from "@/api/types";
import { toScore } from "@/api/utils";

export default defineComponent({
  props: {
    players: {
      type: Array as PropType<
        (Gamer & { userId: number; cards?: number; online?: boolean })[]
      >,
      default: () => [],
    },
  },
  computed: {
    orderedPlayers: function () {
      const players = this.players.map((player) => {
        const score = player.points + toScore(player.cards ?? 0);
        return { score, ...player };
      });
      return players.sort((a, b) => {
        return a.score - b.score;
      });
    },
  },
});
</script>
<style lang="scss">
.player {
  display: flex;
  align-items: center;
  flex-direction: column;
}
</style>
