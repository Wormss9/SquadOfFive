<template>
  <div class="game">
    <OponentsSomethig
      :players="players"
      :ownId="ownId"
      :turn="turn"
      class="third"
    />
    <TableCards :cards="table" class="third" />
    <div
      class="third"
      :style="{
        display: 'flex',
        flexDirection: 'row',
      }"
    >
      <PlayerCards :cards="cards" v-on:cardSelected="updateCards"></PlayerCards>
      <div class="play-buttons">
        <button class="link-like-button" v-on:click="sendPlay">Play</button>
        <button class="link-like-button" v-on:click="sendSkip">Skip</button>
      </div>
    </div>
    <PlayEnd
      v-if="playEnded && !gameEnded"
      :players="players"
      v-on:continue="playEndedFn"
    />
    <GameEnd v-if="gameEnded" :players="players" />
  </div>
</template>

<script lang="ts">
import { get_user, join_game } from "@/api/api";
import { Card, Gamer } from "@/api/types";
import { WsMessage, WsType } from "@/api/receivedMessenges";
import { playMessage, skipMessage } from "@/api/sendMessages";
import PlayerCards from "../components/PlayerCards.vue";
import TableCards from "../components/TableCards.vue";
import OponentsSomethig from "../components/OponentsSomethig.vue";
import PlayEnd from "../components/PlayEnd.vue";
import GameEnd from "../components/GameEnd.vue";
import { get_room_with_users } from "@/api/utils";
import { defineComponent } from "vue";
import { toast } from "vue3-toastify";

export default defineComponent({
  components: {
    PlayerCards,
    OponentsSomethig,
    TableCards,
    PlayEnd,
    GameEnd,
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
      playEnded: false,
      gameEnded: false,
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
      this.websocket = join_game(this.$route.params.ulid as string);
      this.websocket.onmessage = this.handleMessage;
    },
    sendPlay() {
      this.websocket.send(JSON.stringify(playMessage(this.selected)));
    },
    sendSkip() {
      this.websocket.send(JSON.stringify(skipMessage()));
    },
    async handleMessage(event: MessageEvent) {
      const data = JSON.parse(event.data) as WsMessage;
      switch (data.type) {
        case WsType.Online:
          data.message.forEach(this.setOnline);
          break;
        case WsType.Joined:
          this.setOnline(data.message);
          await this.getOwnId();
          this.sortPlayers();
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
        case WsType.EndPlay:
          this.playEnded = true;
          break;
        case WsType.EndGame:
          this.gameEnded = true;
          break;
        case WsType.Error:
          toast.error(data.message.slice(1, -1));
          break;
        default:
          console.log(data);
      }
    },
    async getRoom() {
      this.players = await get_room_with_users(
        this.$route.params.ulid as string
      );
    },
    async getOwnId() {
      this.ownId = (await get_user()).id;
    },
    sortPlayers() {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const turn = this.players.find((p) => p.userId == this.ownId)!.turn;

      this.players = this.players.sort((a, b) => {
        const x = a.turn < turn ? a.turn + 4 : a.turn;
        const y = b.turn < turn ? b.turn + 4 : b.turn;
        return x - y;
      });
    },
    updateCards(cards: Card[]) {
      this.selected = cards;
    },
    playEndedFn() {
      window.location.reload();
    },
  },
  async beforeMount() {
    await this.getRoom();
    await this.getOwnId();
    this.sortPlayers();
    this.setWebsocket();
  },
  onBeforeUnmount() {
    this.websocket.close();
  },
});
</script>
<style lang="scss">
.game {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
}
.game * {
  margin: auto 0;
  align-self: auto;
}
.play-buttons {
  display: flex;
  flex-direction: column;
  flex-grow: 0;
}
</style>
