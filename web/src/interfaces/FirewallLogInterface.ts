import type { RecordId } from "../types/RecordId.ts";

export interface FirewallLogData {
  id?: RecordId | null;
  ip: number[];
  server_ip: string;
  protocol: string;
  source_port: number;
  dest_port: number;
  status: boolean;
  timestamp: string;
}
