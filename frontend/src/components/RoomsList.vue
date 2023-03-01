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
        <router-link :to="'/game/' + room[0].room">Join</router-link>
      </ul>
    </li>
  </ul>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { Rooms } from "@/api/types";
export default defineComponent({
  props: {
    rooms: Object as PropType<Rooms>,
  },
});
</script>
