import { invoke } from "@tauri-apps/api/tauri";
import { AnalyzeTarget } from "./model";

export const scan = async (
  path: string,
  depth: number
): Promise<AnalyzeTarget> => {
  return await invoke("scan", { path, depth });
};
