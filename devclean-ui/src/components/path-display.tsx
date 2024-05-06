import { AnalyzeTarget, AnalyzeTargets } from "@/lib/model";
import * as React from "react";
import prettyBytes from "pretty-bytes";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { DataTable } from "./table/data-table";
import { columns } from "./table/columns";
import { useEffect, useState } from "react";

export function PathDisplay({
  targets,
  className,
}: {
  targets: AnalyzeTargets;
  className: string;
}) {
  const [data, setData] = useState<{ size: string; path: string }[]>([]);
  useEffect(() => {
    setData(
      targets.map((t) => ({
        size: prettyBytes(t.size),
        path: t.path,
      }))
    );
  }, [targets]);
  // make a computed function for targets
  // const computedTargets = () => {

  return (
    <div className="container">
      <DataTable columns={columns} data={data} />
    </div>
    // <ScrollArea className={`w-full rounded-md border ${className}`}>
    //   <div className="p-4">
    //     <h4 className="mb-4 text-sm font-medium leading-none">Paths</h4>
    //     {targets.map((target) => (
    //       <div key={target.path}>
    //         <div className="text-sm">{target.path}</div>
    //         <Separator className="my-2" />
    //       </div>
    //     ))}
    //   </div>
    // </ScrollArea>
  );
}
