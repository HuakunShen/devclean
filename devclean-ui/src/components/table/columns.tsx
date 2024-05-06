import { AnalyzeTarget } from "@/lib/model";
import { ColumnDef } from "@tanstack/react-table";

// This type is used to define the shape of our data.
// You can use a Zod schema here if you want.

export const columns: ColumnDef<{ size: string; path: string }>[] = [
  {
    accessorKey: "path",
    header: "Path",
  },
  {
    accessorKey: "size",
    header: "Size",
  },
];
