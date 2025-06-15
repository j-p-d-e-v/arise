import { createApp } from "vue";
import "./style.css";
import { createWebHashHistory, createRouter } from "vue-router";
import App from "./App.vue";
import CommandExecutionDashboard from "./components/CommandExecutionDashboard.vue";
import FirewallDashboard from "./components/firewall/FirewallDashboard.vue";
const routes = [
  {
    path: "/",
    component: CommandExecutionDashboard,
    name: "CommandExecutionDashboard",
  },
  {
    path: "/firewall",
    component: FirewallDashboard,
    name: "FirewallDashboard",
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

createApp(App).use(router).mount("#app");
