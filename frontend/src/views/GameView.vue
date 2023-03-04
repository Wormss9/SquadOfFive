<template>
  <div class="game">
    <OponentsSomethig :players="players" :ownId="ownId"></OponentsSomethig>
    <TableCards :cards="table"></TableCards>
    <div>
      <PlayerCards :cards="cards" v-on:cardSelected="updateCards"></PlayerCards>
      <div class="buttons">
        <button v-on:click="sendPlay">Play</button>
        <button v-on:click="sendSkip">Skip</button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { get_user, join_room } from "@/api/api";
import { Card, Gamer } from "@/api/types";
import { WsMessage, WsType } from "@/api/receivedMessenges";
import { playMessage, skipMessage } from "@/api/sendMessages";
import PlayerCards from "../components/PlayerCards.vue";
import TableCards from "../components/TableCards.vue";
import OponentsSomethig from "../components/OponentsSomethig.vue";
import { get_room_with_users } from "@/api/utils";
import { defineComponent } from "vue";
import { toast } from "vue3-toastify";

export default defineComponent({
  components: {
    PlayerCards,
    OponentsSomethig,
    TableCards,
  },
  data() {
    return {
      websocket: undefined as unknown as WebSocket,
      players: [] as (Gamer & {
        userId: number;
        cards?: number;
        online?: boolean;
      })[],
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
    setOffline(id: number) {
      this.players.forEach((player) => {
        if (player.id === id) {
          player.online = false;
        }
      });
    },
    setCardAmount(p: [number, number]) {
      const [id, cards] = p;
      this.players.forEach((player) => {
        if (player.id === id) {
          player.cards = cards;
        }
      });
    },
    setWebsocket() {
      this.websocket = join_room(this.$route.params.ulid as string);
      this.websocket.onmessage = this.handleMessage;
    },
    sendPlay() {
      this.websocket.send(JSON.stringify(playMessage(this.selected)));
    },
    sendSkip() {
      this.websocket.send(JSON.stringify(skipMessage()));
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
          break;
        case WsType.Table:
          this.table = data.message;
          break;
        case WsType.Turn:
          this.turn = data.message;
          break;
        case WsType.Left:
          this.setOffline(data.message);
          break;
        case WsType.CardAmmount:
          this.setCardAmount(data.message);
          break;
        case WsType.Error:
          toast.error(data.message.slice(1, -1));
          break;
        default:
          console.log(data);
      }
      console.log("Message from server ", JSON.parse(event.data));
    },
    async getRoom() {
      this.players = await get_room_with_users(
        this.$route.params.ulid as string
      );
    },
    async getOwnId() {
      this.ownId = (await get_user()).id;
    },
    updateCards(cards: Card[]) {
      this.selected = cards;
    },
  },
  async beforeMount() {
    await this.getRoom();
    await this.getOwnId();
    this.setWebsocket();
  },
});
</script>
<style lang="scss">
.game {
  display: flex;
  flex-direction: column;
  height: 100%;
  justify-content: space-around;
}
.game * {
  margin: auto 0;
  align-self: auto;
}
</style>