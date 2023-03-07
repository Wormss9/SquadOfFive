<template>
  <ul class="cards">
    <li v-for="[number, selected, card] in cardsPlus" :key="number">
      <CardImage
        :card="card"
        :selected="selected"
        v-on:click="toggleCardPlus(number)"
        :style="cardStyle(number, cardsPlus.length, selected)"
      ></CardImage>
    </li>
  </ul>
</template>

<script lang="ts">
import { defineComponent, PropType, StyleValue } from "vue";
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
    cardStyle(position: number, am: number, selected: boolean): StyleValue {
      const amount = am - 1;
      const max_rotation = 10;
      return {
        zIndex: position,
        transform: ` translateX(${(amount / 2 - position) * 10}%) rotate(${
          (max_rotation * 2 * position) / amount - max_rotation
        }deg) translateY(${selected ? -25 : 0}px)`,
        boxShadow:
          "rgba(0, 0, 0, 0.09) 0px 8px 4px,\
          rgba(0, 0, 0, 0.09) 0px 16px 8px,\
          rgba(0, 0, 0, 0.09) 0px 32px 16px",
        width: "calc(120%)",
        marginLeft: "calc(-10%)",
        marginRight: "calc(-10%)",
      };
    },
  },
  watch: {
    cards() {
      this.setCardsPlus();
    },
  },
});
</script>
