<template>
  <img :src="cardUrl" />
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { Card } from "@/api/types";
import { be_to_to_color, be_to_to_value } from "@/api/utils";
export default defineComponent({
  props: {
    selected: Boolean,
    card: Object as PropType<Card>,
  },
  computed: {
    cardUrl: function () {
      const images = require.context("../assets/cards", false, /\.png$/);
      const selected = this.selected ? "s" : "";
      return images(
        "./" +
          be_to_to_color(this.card?.color) +
          be_to_to_value(this.card?.value) +
          selected +
          ".png"
      );
    },
  },
});
</script>
