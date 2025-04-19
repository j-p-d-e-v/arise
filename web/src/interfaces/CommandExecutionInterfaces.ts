import type { RecordId } from "../types/RecordId.ts";

export interface CommandExecutionData {
  id: RecordId | null,
  commands: string,
  args: string,
  tgid: number,
  pid: number,
  uid: number,
  gid: number
}


export interface CommandExecutionPaginatedResponse {
  data: CommandExecutionData[],
  limit: number,
  offset: number,
  total: number
}

export interface CommandExecutionStats {
  total: number,
  command: string
}
