import { platform } from "@tauri-apps/api/os";
import { useEffect, useMemo, useState } from "react";

export function PathHighlight({ path }: { path: string }) {
  const [platformName, setPlatformName] = useState<string>();
  const separator = useMemo(
    () => (platformName === "win32" ? "\\" : "/"),
    [platformName]
  );

  useEffect(() => {
    (async () => {
      const p = await platform();
      setPlatformName(p);
    })();
  }, []);
  function getFileName(p: string) {
    return p.split(separator).pop();
  }
  function getDirName(p: string) {
    return p.split(separator).slice(0, -1).join(separator);
  }
  return (
    <pre>
      {getDirName(path)}
      {separator}
      <span className=" text-red-500 dark:text-green-400">
        {getFileName(path)}
      </span>
    </pre>
  );
}
