<template>
  <ul class="table">
    <li v-for="[i, card] in cards?.entries()" :key="i">
      <CardImage :card="card"></CardImage>
    </li>
  </ul>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { Card } from "@/api/types";
import CardImage from "./CardImage.vue";
export default defineComponent({
  components: {
    CardImage,
  },
  props: {
    cards: Array as PropType<Card[]>,
  },
});
</script>
