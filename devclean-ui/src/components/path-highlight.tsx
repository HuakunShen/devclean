export function PathHighlight({ path }: { path: string }) {
  function getFileName(p: string) {
    return p.split("/").pop();
  }
  function getDirName(p: string) {
    return p.split("/").slice(0, -1).join("/");
  }
  return (
    <pre>
      {getDirName(path)}/<span className=" text-red-500 dark:text-green-400">{getFileName(path)}</span>
    </pre>
  );
}
