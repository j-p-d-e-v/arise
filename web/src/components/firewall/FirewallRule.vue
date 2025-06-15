<script setup lang="ts">

import { ref, onMounted } from "vue";
import { FirewallRule } from "../../classes/FirewallRule";
import type { FirewallRuleData, FirewallRuleForm } from "../../interfaces/FirewallRuleInterface";
import { Protocols } from "../../enums/Protocols";
import { RecordId } from "../../types/RecordId";
const rules_loading = ref<boolean>(false);
const rules_data = ref<FirewallRuleData[]>([]);
const rules_timer = ref(null);
const errors = ref<string[]>([]);
const form = ref<FirewallRuleForm>({
  ip: "",
  cidr: 32,
  status: 0,
  protocol: "Tcp"
});
const protocols = ref<string[]>([]);

function getProtocols() {
  protocols.value = Object.entries(Protocols).map((value) => value[1]);
}

function createFirewallRule() {
  errors.value = [];
  const api = new FirewallRule();
  const form_data = form.value;
  const ip: number[] = form.value.ip.split(".").map((value) => parseInt(value));
  if (ip.length != 4) {
    errors.value.push("Invalid ip, it should be 4 octects");
  }
  if (ip.findIndex((value) => value > 255 || value < 0) != -1) {
    errors.value.push("Invalid ip, each octect should be between 0 to 255");
  }
  if (form_data.protocol != Protocols.Icmp && isNaN(form_data.from_port)) {
    errors.value.push("At least from port is required for TCP/UDP protocols.");
  }
  if (!isNaN(form_data.from_port)) {
    if (form_data.from_port < 0 || form_data.from_port > 65535) {
      errors.value.push("Invalid from port, value should be between 0 to 65535");
    }
  }
  if (!isNaN(form_data.to_port)) {
    if (form_data.to_port < 0 || form_data.to_port > 65535) {
      errors.value.push("Invalid to port, value should be between 0 to 65535");
    }
  }
  if (errors.value.length > 0) {
    return false;
  }
  const data: FirewallRuleData = {
    ip: ip,
    from_port: form_data.from_port,
    to_port: form_data.to_port,
    cidr: form_data.cidr,
    status: form_data.status == 1 ? true : false,
    protocol: form_data.protocol
  }

  rules_loading.value = true;
  return api.create(data).then((response) => {
    form.value = {
      ip: "",
      cidr: 32,
      status: 0,
      protocol: "Tcp"
    };
    getFirewallRules();
    rules_loading.value = false;
  }).catch((error) => {
    console.error(error);
    rules_loading.value = false;
  });
}
function getFirewallRules() {
  rules_loading.value = true;
  const api = new FirewallRule();
  return api.list().then((response) => {
    let result: FirewallRuleData[] = response.data;
    rules_data.value = result;
    rules_loading.value = false;
  }).catch((error) => {
    console.error(error);
    rules_loading.value = false;
  });
}

function removeFirewallRule(id: RecordId) {
  rules_loading.value = true;
  const api = new FirewallRule();
  const record_id = `${id.tb}:${id.id.String}`;
  return api.remove(record_id).then(() => {
    getFirewallRules();
    rules_loading.value = false;
  }).catch((error) => {
    console.error(error);
    rules_loading.value = false;
  });
}
onMounted(() => {
  getProtocols();
  getFirewallRules();
});

</script>
<template>
  <div id="firewall-rules text-white">
    <h3 class="mt-3 mb-2 font-semibold text-white uppercase">
      <span v-if="rules_loading" class="loading loading-spinner loading-xs"></span>
      Firewall Rules
    </h3>
    <table class="table text-white border-neutral-300 border-1">
      <thead>
        <tr>
          <th>Protocol</th>
          <th>From Port</th>
          <th>To Port</th>
          <th>Status</th>
          <th>IP</th>
          <th>CIDR</th>
          <th class="w-15"></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(item, key) in rules_data" :key="key">
          <td>{{ item.protocol }}</td>
          <td>{{ item.from_port }}</td>
          <td>{{ item.to_port }}</td>
          <td>
            <span class="text-xs font-semibold text-white badge badge-soft badge-success"
              v-if="item.status">ALLOWED</span>
            <span class="text-xs font-semibold text-white badge based-soft badge-error"
              v-if="!item.status">DENIED</span>
          </td>
          <td>{{ item.ip.join(".") }}</td>
          <td>/{{ item.cidr }}</td>
          <td>
            <button @click="removeFirewallRule(item.id)"
              class="text-red-500 bg-transparent border-0 btn btn-soft btn-xs">
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                class="lucide lucide-circle-x-icon lucide-circle-x">
                <circle cx="12" cy="12" r="10" />
                <path d="m15 9-6 6" />
                <path d="m9 9 6 6" />
              </svg>
            </button>
          </td>
        </tr>
        <tr v-if="errors.length > 0">
          <td colspan="7">
            <div role="alert" class="pl-10 alert alert-error alert-soft">
              <ul class="list-disc list">
                <li v-for="(message, key) in errors" :key="key">{{ message }}</li>
              </ul>
            </div>
          </td>
        </tr>
        <tr>
          <td>
            <select class="select select-xs" v-model="form.protocol">
              <option v-for="(item, key) in protocols" :value="item" :key="key">{{ item }}</option>
            </select>
          </td>
          <td>
            <input type="number" class="w-24 input input-xs" placeholder="From Port" v-model="form.from_port" />
          </td>
          <td>
            <input type="number" class="w-24 input input-xs" placeholder="To Port" v-model="form.to_port" />
          </td>
          <td>
            <select class="select select-xs" v-model="form.status">
              <option value="0">Deny</option>
              <option value="1">Allow</option>
            </select>
          </td>
          <td>
            <input type="text" class="w-50 input input-xs" placeholder="IP Address" v-model="form.ip" />
          </td>
          <td>
            <input type="number" min="0" max="32" class="w-24 input input-xs" placeholder="CIDR"
              v-model.number="form.cidr" />
          </td>
          <td>
            <button @click="createFirewallRule" :disabled="rules_loading"
              class="text-green-500 bg-transparent border-0 btn btn-soft btn-xs">
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                class="lucide lucide-circle-plus-icon lucide-circle-plus">
                <circle cx="12" cy="12" r="10" />
                <path d="M8 12h8" />
                <path d="M12 8v8" />
              </svg> </button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
