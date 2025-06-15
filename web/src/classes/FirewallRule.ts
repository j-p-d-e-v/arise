import type { FirewallRuleData } from "../interfaces/FirewallRuleInterface.ts";
import HttpClient from "./HttpClient.ts";

export class FirewallRule {
  http: HttpClient;
  constructor() {
    this.http = new HttpClient();
  }

  list() {
    return this.http.get_client().get("/firewall-rule/list");
  }

  create(data: FirewallRuleData) {
    return this.http.get_client().post("/firewall-rule/create", data, {
      headers: {
        "Content-Type": "application/json",
      },
    });
  }

  remove(id: string) {
    return this.http.get_client().delete(`/firewall-rule/remove/${id}`);
  }
}
