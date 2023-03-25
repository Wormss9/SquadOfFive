<template>
  <div class="enemy_cards">
    <img
      v-for="number in [...Array(amount ? amount : 16).keys()]"
      :key="number"
      src="../assets/cards/default.png"
      :style="cardStyle(number, amount ? amount : 16)"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent, StyleValue } from "vue";
export default defineComponent({
  props: {
    amount: Number,
  },
  methods: {
    cardStyle(position: number, am: number): StyleValue {
      const amount = am - 1;
      const max_rotation = 30;
      const move = 17.5;
      return {
        transform: ` translateX(${(amount / 2 - position) * move}px) rotate(${
          (max_rotation * 2 * position) / amount - max_rotation
        }deg)`,
        boxShadow: "rgba(0, 0, 0, 0.3) 0px 8px 8px",
        height: "100%",
      };
    },
  },
});
</script>
<style lang="scss">
.enemy_cards {
  display: flex;
  flex-direction: row;
  justify-content: center;
  width: 100%;
  height: 24%;
}
</style>
