import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Button } from "@/components/ui/button";

function App() {
  return (
    <div className="container">
      <Button>Hello</Button>
    </div>
  );
}

export default App;
