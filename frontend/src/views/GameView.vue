<template>
  <div>
    <OponentsSomethig :players="players" :ownId="ownId"></OponentsSomethig>
    <PlayerCards :cards="cards" v-on:cardSelected="updateCards"></PlayerCards>
  </div>
</template>

<script lang="ts">
import { get_user, join_room } from "@/api/api";
import { Card, Gamer } from "@/api/types";
import { WsMessage, WsType } from "@/api/messenges";
import PlayerCards from "../components/PlayerCards.vue";
import OponentsSomethig from "../components/OponentsSomethig.vue";
import { get_room_with_users } from "@/api/utils";
import { defineComponent } from "vue";

export default defineComponent({
  components: {
    PlayerCards,
    OponentsSomethig,
  },
  data() {
    return {
      websocket: undefined as unknown as WebSocket,
      players: [] as (Gamer & { cards?: number; online?: boolean })[],
      table: [] as Card[],
      cards: [] as Card[],
      selected: [] as Card[],
      turn: 0,
      ownId: 0,
    };
  },
  methods: {
    setOnline(id: number) {
      this.players.forEach((player) => {
        if (player.id === id) {
          player.online = true;
        }
      });
    },
    setWebsocket() {
      this.websocket = join_room(this.$route.params.ulid as string);
      this.websocket.onmessage = this.handleMessage;
    },
    handleMessage(event: MessageEvent) {
      const data = JSON.parse(event.data) as WsMessage;
      switch (data.type) {
        case WsType.Online:
          data.message.forEach(this.setOnline);
          break;
        case WsType.Joined:
          this.setOnline(data.message);
          break;
        case WsType.Cards:
          this.cards = data.message;
        // this.setCards(data.message);
      }
      console.log("Message from server ", JSON.parse(event.data));
    },
    async getRoom() {
      this.players = await get_room_with_users(
        this.$route.params.ulid as string
      );
    },
    async getOwnId() {
      const me = await get_user();
      this.ownId = me.id;
    },
    updateCards(cards: Card[]) {
      this.selected = cards;
    },
  },
  async beforeMount() {
    await this.getOwnId();
    await this.getRoom();
    this.setWebsocket();
  },
});
</script>
