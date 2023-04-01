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

const toScore = (ammount: number) => {
  if (ammount == 16) {
    return 80;
  }
  if (ammount >= 14) {
    return ammount * 4;
  }
  if (ammount >= 11) {
    return ammount * 3;
  }
  if (ammount >= 8) {
    return ammount * 2;
  }
  return ammount;
};

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
.gameEnd {
  position: absolute;
  display: flex;
  flex-direction: column;
  align-items: center;
  top: 10%;
  left: 10%;
  width: 80%;
  height: 80%;
  border-radius: 1em;
  box-shadow: black 0px 16px 16px;
  z-index: 1000;
  background-color: #222;
}
.player {
  display: flex;
  align-items: center;
  flex-direction: column;
}
</style>
