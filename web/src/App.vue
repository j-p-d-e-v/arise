<script setup lang="ts">
import CommandExecutionDashboard from "./components/CommandExecutionDashboard.vue";

</script>

<template>
  <div class="grid grid-cols-[20%_80%] h-full">
    <div class="p-3 h-full bg-stone-800">Nav</div>
    <div class="h-full bg-stone-100 text-stone-900">
      <CommandExecutionDashboard />
    </div>
  </div>
</template>

<style scoped></style>
