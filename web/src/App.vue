<script setup lang="ts">
import { ref } from "vue";
import CommandExecutionDashboard from "./components/CommandExecutionDashboard.vue";
import FirewallDashboard from "./components/firewall/FirewallDashboard.vue";
import { useRouter } from "vue-router";
import icon from "./assets/icon.gif";
const router = useRouter();

function goTo(name: string) {
  console.log(router);
  router.push({ name: name })
}

</script>

<template>
  <div class="grid grid-cols-[250px_80%] h-full">
    <div class="relative p-2">
      <div align="center">
        <img :src="icon" height="60" />
      </div>
      <h3 class="mb-2 text-sm font-bold" align="center">PROJECT ARISE</h3>
      <ul class="w-full menu bg-base-200 rounded-box">
        <li @click="goTo('CommandExecutionDashboard')"><a>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
              class="lucide lucide-square-terminal-icon lucide-square-terminal">
              <path d="m7 11 2-2-2-2" />
              <path d="M11 13h4" />
              <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
            </svg>
            Command Execution Dashboard</a></li>
        <li @click="goTo('FirewallDashboard')"><a>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
              class="lucide lucide-brick-wall-fire-icon lucide-brick-wall-fire">
              <path d="M16 3v2.107" />
              <path
                d="M17 9c1 3 2.5 3.5 3.5 4.5A5 5 0 0 1 22 17a5 5 0 0 1-10 0c0-.3 0-.6.1-.9a2 2 0 1 0 3.3-2C13 11.5 16 9 17 9" />
              <path d="M21 8.274V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.938" />
              <path d="M3 15h5.253" />
              <path d="M3 9h8.228" />
              <path d="M8 15v6" />
              <path d="M8 3v6" />
            </svg>
            Firewall Dashboard</a></li>
      </ul>
    </div>
    <div class="p-3 h-full text-neutral-900">
      <RouterView />
    </div>
  </div>
</template>

<style scoped></style>
