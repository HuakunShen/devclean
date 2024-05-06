import "./App.css";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { ThemeProvider } from "@/components/theme-provider";
import { ModeToggle } from "./components/mode-toggle";
import { scan } from "@/lib/command";
import { PathDisplay } from "@/components/path-display";
import { AnalyzeTarget, AnalyzeTargets } from "./lib/model";
import { DisplayTable } from "./components/table/display-table";

function App() {
  const [targets, setTargets] = useState<AnalyzeTarget[]>([]);

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <div className="container bg-background text-foreground flex flex-col max-h-screen">
        <Button
          onClick={() => {
            scan("/Users/hacker/Dev/projects/", 10)
              .then((res) => {
                console.log(res);
                setTargets(AnalyzeTargets.parse(res));
              })
              .catch(console.error);
          }}
        >
          Hello
        </Button>
        <ModeToggle />
        <PathDisplay targets={targets} className="h-96" />
        <DisplayTable data={targets} />
      </div>
    </ThemeProvider>
  );
}

export default App;
