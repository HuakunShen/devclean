import * as React from "react";
import {
  ColumnDef,
  ColumnFiltersState,
  SortingState,
  VisibilityState,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { ArrowUpDown, ChevronDown, MoreHorizontal } from "lucide-react";
import { writeText, readText } from "@tauri-apps/api/clipboard";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { AnalyzeTargets, PathDisplayRow } from "@/lib/model";
import { useEffect, useState } from "react";
import prettyBytes from "pretty-bytes";
import { delete_dir } from "@/lib/command";
import { toast, useToast } from "@/components/ui/use-toast";
import { Progress } from "@/components/ui/progress";

export const columns: ColumnDef<PathDisplayRow>[] = [
  {
    id: "select",
    header: ({ table }) => (
      <Checkbox
        checked={
          table.getIsAllPageRowsSelected() ||
          (table.getIsSomePageRowsSelected() && "indeterminate")
        }
        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
        aria-label="Select all"
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={(value) => row.toggleSelected(!!value)}
        aria-label="Select row"
      />
    ),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: "path",
    header: "Path",
    cell: ({ row }) => <div>{row.getValue("path")}</div>,
  },
  {
    accessorKey: "size",
    header: "Size",
    // header: ({ column }) => {
    //   return (
    //     <div className="text-center">
    //       <Button
    //         variant="ghost"
    //         onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
    //       >
    //         Size
    //         <ArrowUpDown className="ml-2 h-4 w-4" />
    //       </Button>
    //     </div>
    //   );
    // },
    cell: ({ row }) => (
      <div className="font-medium">{row.getValue("size")}</div>
    ),
  },
  {
    id: "actions",
    enableHiding: false,
    cell: ({ row }) => {
      const { toast } = useToast();

      return (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" className="h-8 w-8 p-0">
              <span className="sr-only">Open menu</span>
              <MoreHorizontal className="h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuLabel>Actions</DropdownMenuLabel>
            <DropdownMenuItem
              onClick={() => {
                writeText(row.getValue("path"))
                  .then(() => {
                    toast({
                      title: "Copied",
                      description: "Path copied to clipboard",
                    });
                  })
                  .catch(() => {
                    toast({
                      title: "destructive",
                      description: "Failed to copy path to clipboard",
                    });
                  });
              }}
            >
              Copy Path
            </DropdownMenuItem>
            <DropdownMenuItem
              onClick={() => {
                delete_dir(row.getValue("path")).then(() => {
                  toast({
                    variant: "destructive",
                    title: "Deleted",
                    description: "Folder Deleted",
                  });
                });
              }}
            >
              Delete Directory
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      );
    },
  },
];

export function DisplayTable({ data }: { data: AnalyzeTargets }) {
  const [sorting, setSorting] = React.useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>(
    []
  );
  const [columnVisibility, setColumnVisibility] =
    React.useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = React.useState({});
  const [deleteProgress, setDeleteProgress] = React.useState(0);
  const [dataDisplay, setDataDisplay] = useState<PathDisplayRow[]>([]);
  useEffect(() => {
    setDataDisplay(
      data.map((t) =>
        PathDisplayRow.parse({
          bytes: t.size,
          size: prettyBytes(t.size),
          path: t.path,
        })
      )
    );
  }, [data]);

  const table = useReactTable({
    data: dataDisplay,
    columns,
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    onRowSelectionChange: setRowSelection,
    state: {
      sorting,
      columnFilters,
      columnVisibility,
      rowSelection,
    },
  });

  return (
    <div className="w-full">
      <div className="flex items-center py-4">
        <Input
          placeholder="Filter path..."
          value={(table.getColumn("path")?.getFilterValue() as string) ?? ""}
          onChange={(event) =>
            table.getColumn("path")?.setFilterValue(event.target.value)
          }
          className="max-w-sm"
        />
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" className="ml-auto">
              Columns <ChevronDown className="ml-2 h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            {table
              .getAllColumns()
              .filter((column) => column.getCanHide())
              .map((column) => {
                return (
                  <DropdownMenuCheckboxItem
                    key={column.id}
                    className="capitalize"
                    checked={column.getIsVisible()}
                    onCheckedChange={(value) =>
                      column.toggleVisibility(!!value)
                    }
                  >
                    {column.id}
                  </DropdownMenuCheckboxItem>
                );
              })}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead key={header.id}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
                    </TableHead>
                  );
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && "selected"}
                >
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id} className="py-0">
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className="h-24 text-center"
                >
                  No results.
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
      <div className="flex items-center justify-end space-x-2 py-4">
        <div className="flex-1 text-sm text-muted-foreground">
          {table.getFilteredSelectedRowModel().rows.length} of{" "}
          {table.getFilteredRowModel().rows.length} row(s) selected.
        </div>
        <div className="space-x-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.previousPage()}
            disabled={!table.getCanPreviousPage()}
          >
            Previous
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.nextPage()}
            disabled={!table.getCanNextPage()}
          >
            Next
          </Button>
          <Button
            variant="destructive"
            onClick={async () => {
              setDeleteProgress(0);
              const rows = table.getFilteredSelectedRowModel().rows;
              const interval = 100.0 / rows.length;
              const jobs = rows.map(
                (row) =>
                  new Promise((resolve) => {
                    delete_dir(row.getValue("path")).then(() => {
                      resolve(setDeleteProgress((prev) => prev + interval));
                    });
                  })
              );
              Promise.all(jobs).then(() => {
                setTimeout(() => {
                  setDeleteProgress(0);
                }, 2000);
                toast({
                  variant: "destructive",
                  title: "Deleted",
                  description: "Folders Deleted",
                });
              });
            }}
          >
            Delete Selected
          </Button>
        </div>
      </div>
      {deleteProgress > 0 && (
        <Progress value={deleteProgress} className="w-full" />
      )}
    </div>
  );
}
