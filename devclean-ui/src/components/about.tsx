import { getVersion } from "@tauri-apps/api/app";
import { useState } from "react";
import { Button } from "./ui/button";
import { open } from "@tauri-apps/api/shell";

function BtnLink({
  href,
  children,
}: {
  href: string;
  children: React.ReactNode;
}) {
  return (
    <Button
      variant="link"
      onClick={() => {
        open(href);
      }}
    >
      {children}
    </Button>
  );
}

export function About({ className }: { className?: string }) {
  const [version, setVersion] = useState<string | null>(null);
  getVersion().then((v) => setVersion(v));

  return (
    <div className={`flex flex-col space-y-3 mt-4 ${className}`}>
      <p>
        <strong className="font-bold">App Name:{"  "}</strong>DevClean-UI
      </p>
      <p>
        <strong className="font-bold">App Version:{"  "}</strong>
        {version}
      </p>
      <p>
        <strong>Source Code: </strong>
        <BtnLink href="https://github.com/HuakunShen/devclean.git">
          https://github.com/HuakunShen/devclean.git
        </BtnLink>
      </p>
      <p>
        <BtnLink href="https://github.com/HuakunShen/devclean/releases/latest">
          Latest Release
        </BtnLink>
      </p>
      <p>
        Auto Updater is configured. Every time a new release is published your
        app will ask you whether you want to update to the latest release. Auto
        update is in just one click.
      </p>
      <p>
        <strong>Author: </strong>
        <BtnLink href="https://github.com/HuakunShen">
          https://github.com/HuakunShen
        </BtnLink>
      </p>
    </div>
  );
}
