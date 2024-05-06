import { z } from "zod";

export const AnalyzeTarget = z.object({
  path: z.string(),
  size: z.number().positive(),
  depth: z.number().positive(),
});
export type AnalyzeTarget = z.infer<typeof AnalyzeTarget>;
export const AnalyzeTargets = AnalyzeTarget.array();
export type AnalyzeTargets = z.infer<typeof AnalyzeTargets>;
