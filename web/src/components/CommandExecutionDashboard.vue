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
  getStats();
  configureChart();
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
        barWidth: 20,
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
    let result: CommandExecutionStats[] = response.data;

    stats_data.value = result;
    if (result.length > 0) {
      if (chart.value == null) {
      }
    }
    if (chart.value !== null) {
      updateChart(result);
    }
    setTimeout(() => getStats(), 500);
  }).catch((error) => {
    console.error(error);
    stats_loader.value = false;
  })
}

function getExecutedCommands() {
  pagination_loader.value = true;
  const api = new CommandExecution();
  api.list(offset.value, limit.value).then((response) => {
    let data: CommandExecutionPaginatedResponse = response.data;
    paginated_data.value = data;

    setTimeout(() => getExecutedCommands(), 1000);
  }).catch((error) => {
    console.error(error);
  }).finally(() => {
    pagination_loader.value = false;
  });
}

</script>

<template>
  <div class="pb-2 mb-5 text-2xl text-white border-b-3">
    <span class="float-left mt-[2px]">
      <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none"
        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
        class="lucide lucide-square-terminal-icon lucide-square-terminal">
        <path d="m7 11 2-2-2-2" />
        <path d="M11 13h4" />
        <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
      </svg>
    </span>
    <span class="float-left ml-3">
      COMMAND EXECUTION
    </span>
    <div style="clear:both;"></div>
  </div>
  <div class="h-full" id="command-execution-container">
    <div class="rounded-sm">
      <h3 class="pb-1 font-semibold text-left text-white align-center">
        Command Execution Counts Chart
      </h3>
      <div class="border-1 border-neutral-300">
        <div id="command-execution-chart" class="w-full h-full border-1"></div>
      </div>
      <div class="content-center pb-15 h-[30px]" align="center">
        <span class="text-white loading loading-spinner loading-sm" v-if="stats_loader"></span>
      </div>
    </div>
    <template v-if="paginated_data">
      <div class="mt-3 rounded-sm">
        <h3 class="p-0 mb-3 font-semibold text-white">Recently Executed Commands</h3>
        <table class="table text-white border-1 table-sm">
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
</template>

<style scoped>
#command-execution-container {}

#command-execution-chart {
  width: auto;
  min-height: 700px;
}
</style>
