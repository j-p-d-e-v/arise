import axios from "axios";
import type { AxiosInstance } from "axios";
import config from "../config.json";

export default class HttpClient {
  client: AxiosInstance;

  constructor() {
    this.client = axios.create({
      baseURL: config.api_base_url
    });

  }

  get_client(): AxiosInstance {
    return this.client
  }
}
