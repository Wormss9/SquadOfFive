<template>
  <div class="gameEnd">
    <BarChart class="endChart" v-bind="barChartProps" />
    <button @click="next" class="link-like-button button">Continue</button>
  </div>
</template>

<script lang="ts">
import { computed } from "vue";
import { defineComponent, PropType } from "vue";
import { BarChart, useBarChart } from "vue-chart-3";
import { Gamer } from "@/api/types";
import { Chart, ChartData, ChartOptions, registerables } from "chart.js";

Chart.register(...registerables);

const toScore = (ammount: number) => {
  if (ammount == 16) {
    return 80;
  }
  if (ammount >= 14) {
    return ammount * 4;
  }
  if (ammount >= 11) {
    return ammount * 3;
  }
  if (ammount >= 8) {
    return ammount * 2;
  }
  return ammount;
};

const toColor = (ammount: number) => {
  if (ammount == 16) {
    return "#000";
  }
  if (ammount >= 14) {
    return "#00F";
  }
  if (ammount >= 11) {
    return "#0F0";
  }
  if (ammount >= 8) {
    return "#F00";
  }
  return "#FFF";
};

export default defineComponent({
  components: {
    BarChart,
  },
  props: {
    players: {
      type: Array as PropType<
        (Gamer & { userId: number; cards?: number; online?: boolean })[]
      >,
      default: () => [],
    },
  },
  data() {
    const testData = computed<ChartData<"bar">>(() => ({
      labels: this.players.map((player) => player.nick),
      datasets: [
        {
          data: this.players.map((player) => toScore(player.cards ?? 0)),
          backgroundColor: this.players.map((player) =>
            toColor(player.cards ?? 0)
          ),
        },
      ],
    }));
    const options = computed<ChartOptions<"bar">>(() => ({
      plugins: {
        legend: {
          display: false,
        },
        tooltip: {
          enabled: false,
        },
      },
    }));
    const { barChartProps, barChartRef } = useBarChart({
      chartData: testData,
      options,
    });

    return {
      barChartProps,
      barChartRef,
    };
  },
  methods: {
    next() {
      this.$emit("continue");
    },
  },
  computed: {},
});
</script>
<style lang="scss">
.gameEnd {
  position: absolute;
  display: flex;
  flex-direction: column;
  align-items: center;
  top: 10%;
  left: 10%;
  width: 80%;
  height: 80%;
  border-radius: 1em;
  box-shadow: black 0px 16px 16px;
  z-index: 1000;
  background-color: #222;
}
.endChart {
  width: 100%;
  height: 80%;
}
</style>
