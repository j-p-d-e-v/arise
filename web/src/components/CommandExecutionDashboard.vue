<script setup lang="ts">
import { ref, onMounted } from "vue";
import { CommandExecution } from "../classes/CommandExecution.ts";
import { CommandExecutionStats, CommandExecutionPaginatedResponse, CommandExecutionData } from "../interfaces/CommandExecutionInterfaces.ts";
import * as echarts from 'echarts';
import echarts_theme from "../assets/echarts-theme.json";

const stats_loader = ref<bool>(false);
const pagination_loader = ref<bool>(false);
const stats_data = ref<CommandExecutionStats[]>([]);
const paginated_data = ref<CommandExecutionPaginatedResponse | null>(null);
const offset = ref<number>(0);
const limit = ref<number>(10);
const chart = ref(null);
echarts.registerTheme("echarts_theme", echarts_theme);

onMounted(() => {
  configureChart();
  getStats();
  getExecutedCommands();
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
  stats_loader.value = true;
  const api = new CommandExecution();
  api.stats().then((response) => {
    let data: CommandExecutionStats[] = response.data;
    stats_data.value = data;
    updateChart(data);
    setTimeout(() => getStats(), 500);
  }).catch((error) => {
    console.error(error);
  }).finally(() => {
    stats_loader.value = false;
  });
}

function getExecutedCommands() {
  pagination_loader.value = true;
  const api = new CommandExecution();
  api.list(offset.value, limit.value).then((response) => {
    let data: CommandExecutionPaginatedResponse = response.data;
    paginated_data.value = data;

    setTimeout(() => getExecutedCommands(), 1500);
  }).catch((error) => {
    console.error(error);
  }).finally(() => {
    pagination_loader.value = false;
  });
}

</script>

<template>
  <div class="p-3 h-full" id="command-execution-container">
    <div class="rounded-sm border-1 border-stone-600">
      <h3 class="p-3 pt-6 pb-0 text-xl font-semibold text-center text-white align-center">Command Execution Counts</h3>
      <div id="command-execution-chart"></div>
      <template v-if="paginated_data">
        <h3 class="p-3 pb-0 font-semibold text-white">Recently Executed Commands</h3>
        <div class="p-3">
          <table class="table text-white border-1 border-stone-600 table-sm">
            <thead>
              <tr>
                <th>Command</th>
                <th>Arguments</th>
                <th>PID</th>
                <th>TGID</th>
                <th>GID</th>
                <th>UID</th>
                <th>Timestamp</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(item, key) in paginated_data.data" :key="key">
                <th>{{ item.command }}</th>
                <td>{{ item.args }}</td>
                <td>{{ item.pid }}</td>
                <td>{{ item.tgid }}</td>
                <td>{{ item.gid }}</td>
                <td>{{ item.uid }}</td>
                <td>{{ item.timestamp }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </template>
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
