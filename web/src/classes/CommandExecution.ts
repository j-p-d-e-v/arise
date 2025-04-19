import HttpClient from "./HttpClient.ts";

export class CommandExecution {
  http: HttpClient;
  constructor() {
    this.http = new HttpClient();
  }

  stats() {
    return this.http.get_client().get("/command-execution/stats");
  }
  list(offset: number, limit: number) {
    return this.http.get_client().get("/command-execution/list", {
      params: {
        limit: limit,
        offset: offset
      }
    });
  }
}
