<template>
  <ul class="cards">
    <li v-for="[number, selected, card] in cardsPlus" :key="number">
      <CardImage
        :card="card"
        :selected="selected"
        v-on:click="toggleCardPlus(number)"
      ></CardImage>
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
  data() {
    return {
      cardsPlus: [] as [number, boolean, Card][],
    };
  },
  methods: {
    setCardsPlus() {
      if (!this.cards) {
        return;
      }
      let i = -1;
      const numberedCards = this.cards.map((card) => {
        i += 1;
        return [i, false, card] as [number, boolean, Card];
      });
      this.cardsPlus = numberedCards;
    },
    toggleCardPlus(number: number) {
      this.cardsPlus = this.cardsPlus.map((card) => {
        if (card[0] === number) {
          return [card[0], !card[1], card[2]];
        }
        return card;
      });
      this.$emit(
        "cardSelected",
        this.cardsPlus.filter(([, state]) => state).map(([, , card]) => card)
      );
    },
  },
  watch: {
    cards() {
      this.setCardsPlus();
    },
  },
});
</script>
