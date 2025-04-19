<script setup lang="ts">
import { ref, onMounted } from "vue";
import { CommandExecution } from "../classes/CommandExecution.ts";
import { CommandExecutionStats } from "../interfaces/CommandExecutionInterfaces.ts";
import * as echarts from 'echarts';
import echarts_theme from "../assets/echarts-theme.json";

const loader = ref<bool>(false);
const stats_data = ref<CommandExecutionStats[]>([]);
const chart = ref(null);
echarts.registerTheme("echarts_theme", echarts_theme);

onMounted(() => {
  configureChart();
  getStats();
});
function configureChart(data: CommandExecutionStats[]) {
  chart.value = echarts.init(document.getElementById("command-execution-chart"), "echarts_theme");
  chart.value.setOption({
    title: {
      text: "Command Execution Stats",
      show: false,
    },
    tooltip: {},
    xAxis: {
    },
    grid: {
      containLabel: true
    },
    yAxis: {},
    series: []
  });

}
function updateChart(data: CommandExecutionStats[]) {
  chart.value.setOption({
    yAxis: {
      type: "category",
      axisLabel: {
        inside: true,
        color: "#FFF",
      },
      z: 10,
      data: data.map((item) => item.command)
    },
    series: [
      {
        name: "Stats",
        type: "bar",
        showBackground: true,
        realtimeSort: true,
        barWidth: 30,
        large: true,
        barGap: "30%",
        backgroundStyle: {
        },
        colorBy: "data",
        data: data.map((item) => item.total)
      }
    ]

  });

}



function getStats() {

  loader.value = true;
  const api = new CommandExecution();
  api.stats().then((response) => {
    let data: CommandExecutionStats[] = response.data;
    stats_data.value = data;
    updateChart(data);
    setTimeout(() => getStats(), 500);
  }).catch((error) => {

  }).finally(() => {
    loader.value = false;
  });
}
</script>

<template>
  <div class="p-3 h-full" id="command-execution-container">
    <div class="rounded-sm border-1 border-stone-600">
      <h3 class="p-3 pt-6 pb-0 text-xl font-semibold text-center text-white align-center">Command Execution Counts</h3>
      <div id="command-execution-chart"></div>
    </div>
  </div>
</template>

<style scoped>
#command-execution-container {
  background-color: rgba(51, 51, 51, 1);
}

#command-execution-chart {
  width: auto;
  height: 900px;
}
</style>
