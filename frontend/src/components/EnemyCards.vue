<template>
  <div class="enemy_cards">
    <img
      v-for="number in [...Array(amount ? amount : 0).keys()]"
      :key="number"
      src="../assets/cards/default.png"
      :style="cardStyle(number, amount ? amount : 0)"
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
        zIndex: position,
        height: "3rem",
        transform: ` translateX(${(amount / 2 - position) * move}px) rotate(${
          (max_rotation * 2 * position) / amount - max_rotation
        }deg)`,
        "box-shadow":
          //TODO!
          "rgba(0, 0, 0, 0.15) 0px -36px 30px 0px inset,\
           rgba(0, 0, 0, 0.15) 0px -79px 40px 0px inset,\
           rgba(0, 0, 0, 0.06) 0px 2px 1px,\
           rgba(0, 0, 0, 0.09) 0px 4px 2px,\
           rgba(0, 0, 0, 0.09) 0px 8px 4px,\
           rgba(0, 0, 0, 0.09) 0px 16px 8px,\
           rgba(0, 0, 0, 0.09) 0px 32px 16px",
      };
    },
  },
});
</script>
<style lang="scss">
.enemy_cards {
  display: flex;
  flex-direction: row;
  align-items: center;
}
</style>
