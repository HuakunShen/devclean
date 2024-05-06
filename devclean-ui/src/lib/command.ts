import { invoke } from "@tauri-apps/api/tauri";
import { AnalyzeTarget } from "./model";

export function scan(path: string, depth: number): Promise<AnalyzeTarget> {
  return invoke("scan", { path, depth });
}

export function delete_dir(path: string): Promise<void> {
  return invoke("delete_dir", { path });
}

export function pathExists(path: string): Promise<boolean> {
  return invoke("path_exists", { path });
}
