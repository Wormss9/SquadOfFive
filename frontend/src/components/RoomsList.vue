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
          <ul v-if="owned && room.length !== 4">
            <button v-on:click="copyLink(player.room)">Join link</button>
          </ul>
        </li>
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
      console.log(ulid);
      await toClipboard(`${window.location.origin}/join/${ulid}`);
      toast.info("Link copied to clippboard");
    },
  },
});
</script>
