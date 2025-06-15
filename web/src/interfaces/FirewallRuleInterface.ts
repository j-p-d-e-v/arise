import type { RecordId } from "../types/RecordId.ts";
import { Protocols } from "../enums/Protocols.ts";
export interface FirewallRuleData {
  id?: RecordId | null;
  ip: number[];
  protocol: string;
  from_port?: number;
  to_port?: number;
  status: boolean;
  cidr: number;
}

export interface FirewallRuleForm {
  ip: string;
  protocol: Protocols;
  from_port?: number;
  to_port?: number;
  status: 1 | 0;
  cidr: number;
}
