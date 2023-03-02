<template>
  <ul class="oponents">
    <li
      v-for="player in players.filter((p) => p.userId != ownId)"
      :key="player.id"
    >
      <div class="enemy">
        <div>{{ player.nick }}</div>
        <img
          v-bind:src="'data:image/png;base64,' + player.avatar"
          class="avatar"
        />
        <div>{{ player.points }}</div>
      </div>
      <EnemyCards :amount="player.cards" />
    </li>
  </ul>
</template>

<script lang="ts">
import { Gamer } from "@/api/types";
import EnemyCards from "../components/EnemyCards.vue";
import { defineComponent, PropType } from "vue";

export default defineComponent({
  components: {
    EnemyCards,
  },
  props: {
    players: {
      type: Array as PropType<
        (Gamer & { userId: number; cards?: number; online?: boolean })[]
      >,
      default: () => [],
    },
    ownId: Number,
  },
});
</script>
