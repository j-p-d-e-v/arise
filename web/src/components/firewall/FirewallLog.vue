<script setup lang="ts">

import { ref, onMounted } from "vue";
import { FirewallLog } from "../../classes/FirewallLog";
import type { FirewallLogData } from "../../interfaces/FirewallLogInterface";
const logs_loading = ref<boolean>(false);
const logs_data = ref<FirewallLogInterface[]>([]);
const logs_timer = ref(null);
function getFirewallLogs() {
  logs_loading.value = true;
  const api = new FirewallLog();
  return api.list(20).then((response) => {
    let result: FirewallLogData[] = response.data;
    logs_data.value = result;
    logs_loading.value = false;
    logs_timer.value = setTimeout(() => {
      getFirewallLogs();
    }, 1000);
  }).catch((error) => {
    console.error(error);
    logs_loading.value = false;
  });
}
onMounted(() => {
  getFirewallLogs();
});

</script>
<template>
  <div id="firewall-logs text-white">
    <h3 class="mt-3 mb-2 font-semibold text-white uppercase">
      Firewall Logs (BLOCKED CONNECTIONS)
      <span v-if="logs_loading" class="loading loading-spinner loading-xs"></span>
    </h3>
    <div class="overflow-y-auto border-1 border-neutral-300 h-140">
      <table class="table text-white border-0">
        <thead>
          <tr>
            <th>Procotol</th>
            <th>Device IP</th>
            <th>Device Port</th>
            <th>Server Ip</th>
            <th>Server Port</th>
            <th>Timestamp</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(item, key) in logs_data" :key="key">
            <td>{{ item.protocol }}</td>
            <td>{{ item.ip.join(".") }}</td>
            <td>{{ item.source_port }}</td>
            <td>{{ item.server_ip }}</td>
            <td>{{ item.dest_port }}</td>
            <td>{{ item.timestamp }}</td>
          </tr>
        </tbody>
      </table>
    </div>

  </div>
</template>
