import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { ModeToggle } from "@/components/mode-toggle";
import { pathExists, scan } from "@/lib/command";
import { AnalyzeTarget, AnalyzeTargets } from "@/lib/model";
import { DisplayTable } from "@/components/table/display-table";
import { open } from "@tauri-apps/api/dialog";
import { z } from "zod";
import { useToast } from "@/components/ui/use-toast";
import {
  ReloadIcon,
  FileIcon,
  MagnifyingGlassIcon,
} from "@radix-ui/react-icons";
import { AboutDrawer } from "../about-drawer";
import { AboutSheet } from "../about-sheet";

export function HomePage() {
  const [targets, setTargets] = useState<AnalyzeTarget[]>([]);
  const [pickedFolder, setPickedFolder] = useState<string>("");
  const [scanning, setScanning] = useState(false);
  const { toast } = useToast();

  useEffect(() => {
    const cacheScanFolder = localStorage.getItem("cache-scan-folder");
    if (cacheScanFolder) {
      setPickedFolder(cacheScanFolder);
    }
  }, []);

  useEffect(() => {
    localStorage.setItem("cache-scan-folder", pickedFolder);
  }, [pickedFolder]);

  return (
    <>
      <div data-tauri-drag-region className="h-8" />
      <div className="container bg-background text-foreground flex flex-col max-h-screen py-2 space-y-4">
        <div className="flex space-x-2">
          <div className="flex justify-between w-full">
            <div className="flex space-x-2">
              <Button
                variant="outline"
                onClick={() => {
                  open({ directory: true })
                    .then((res) => {
                      setPickedFolder(z.string().parse(res));
                    })
                    .catch(console.error);
                }}
              >
                <FileIcon className="mr-2 h-4 w-4" />
                Pick Folder
              </Button>
              <Button
                disabled={scanning}
                variant="secondary"
                className="dark:bg-green-700 dark:hover:bg-green-600 bg-green-600 text-white hover:bg-green-500"
                onClick={async () => {
                  setScanning(true);
                  const exists = await pathExists(pickedFolder)
                    .then((exists) => {
                      if (exists) {
                      } else {
                        toast({
                          variant: "destructive",
                          title: "Path Not Found",
                        });
                        setScanning(false);
                      }
                      return exists;
                    })
                    .catch((err) => {
                      toast({
                        variant: "destructive",
                        title: "Error",
                        description: err,
                      });
                    });
                  if (exists) {
                    await scan(pickedFolder, 10)
                      .then((res) => {
                        setTargets(AnalyzeTargets.parse(res));
                      })
                      .catch(console.error);
                    setScanning(false);
                  }
                }}
              >
                {scanning ? (
                  <ReloadIcon className="mr-2 h-4 w-4 animate-spin" />
                ) : (
                  <MagnifyingGlassIcon className="mr-2 h-4 w-4" />
                )}
                Scan
              </Button>
            </div>
            <div className="flex space-x-2">
              <ModeToggle />
              <AboutSheet />
              {/* <AboutDrawer /> */}
            </div>
          </div>
        </div>
        {pickedFolder && (
          <div>
            <strong className="font-bold">Picked Path:</strong>{" "}
            <pre className="inline">{pickedFolder}</pre>
          </div>
        )}
        <DisplayTable data={targets} />
      </div>
    </>
  );
}
