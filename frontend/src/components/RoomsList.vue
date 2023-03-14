<template>
  <ul class="rooms">
    <li v-for="room in rooms" :key="room[0].room">
      <ul class="room">
        <li v-for="player in room" :key="player.id">
          <ul>
            <div>{{ player.nick }}</div>
            <img
              v-bind:src="'data:image/png;base64,' + player.avatar"
              class="avatar"
            />
            <div>{{ player.points }}</div>
          </ul>
        </li>
        <button
          v-if="owned"
          class="link-like-delete-button right"
          v-on:click="deleteRoom(room[0].room)"
        >
          Delete
        </button>
        <button class="link-like-button" v-on:click="copyLink(room[0].room)">
          Join link
        </button>
        <router-link :to="'/game/' + room[0].room">Join</router-link>
      </ul>
    </li>
  </ul>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { Rooms } from "@/api/types";
import useClipboard from "vue-clipboard3";
import { toast } from "vue3-toastify";
import { delete_room } from "@/api/api";
export default defineComponent({
  props: {
    rooms: Object as PropType<Rooms>,
    owned: {
      type: Boolean,
      default: false,
    },
  },
  methods: {
    async copyLink(ulid: string) {
      const { toClipboard } = useClipboard();
      await toClipboard(`${window.location.origin}/join/${ulid}`);
      toast.info("Link copied to clipboard");
    },
    async deleteRoom(ulid: string) {
      await delete_room(ulid);
      location.assign("rooms");
    },
  },
});
</script>
