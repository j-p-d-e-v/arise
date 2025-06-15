import HttpClient from "./HttpClient.ts";

export class FirewallLog {
  http: HttpClient;
  constructor() {
    this.http = new HttpClient();
  }

  list(limit: number) {
    return this.http.get_client().get("/firewall-log/list", {
      params: {
        limit: limit,
      },
    });
  }
}
