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
import { toScore, toColor } from "@/api/utils";

Chart.register(...registerables);

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
.endChart {
  width: 100%;
  height: 80%;
}
</style>
