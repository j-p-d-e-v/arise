<script setup lang="ts">
import FirewallLog from "./FirewallLog.vue";
import FirewallRule from "./FirewallRule.vue";

</script>
<template>
  <div class="pb-2 text-2xl text-white border-b-3">
    <span class="float-left mt-[1px]">
      <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none"
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
    </span>
    <span class="float-left ml-3">
      FIREWALL
    </span>
    <div style="clear:both;"></div>
  </div>
  <div id="firewall-rules">
    <FirewallRule />
  </div>
  <div id="firewall-logs">
    <FirewallLog />
  </div>
</template>
