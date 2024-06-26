import "./App.css";
import { ThemeProvider } from "@/components/theme-provider";
import { Toaster } from "@/components/ui/toaster";
import { HomePage } from "./components/pages/home";
import { useEffect } from "react";
import { getCurrent } from "@tauri-apps/api/window";

function App() {
  useEffect(() => {
    getCurrent().show();
  }, []);
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <Toaster />
      <HomePage />
    </ThemeProvider>
  );
}

export default App;
