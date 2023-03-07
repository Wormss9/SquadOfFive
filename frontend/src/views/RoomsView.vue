<template>
  <div>
    <button v-on:click="createRoom()" class="link-like-button">Create</button>
    <RoomsList :owned="true" v-bind:rooms="rooms.owned" />
    <RoomsList v-bind:rooms="rooms.joined" />
  </div>
</template>

<script lang="ts">
import { create_room } from "@/api/api";
import { User, Player } from "@/api/types";
import { get_rooms_with_users } from "@/api/utils";
import { defineComponent } from "vue";
import RoomsList from "../components/RoomsList.vue";

export default defineComponent({
  components: {
    RoomsList,
  },
  data() {
    return {
      rooms: {
        joined: [] as (User & Player)[][],
        owned: [] as (User & Player)[][],
      },
    };
  },
  methods: {
    getRooms: async function () {
      this.rooms = await get_rooms_with_users();
    },
    createRoom: async function () {
      console.log("asd");
      await create_room();
      location.assign("rooms");
    },
  },
  beforeMount() {
    this.getRooms();
  },
});
</script>
