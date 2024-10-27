import React from "react";
import { Button } from "./ui/button";
import { Pencil2Icon } from "@radix-ui/react-icons";
import Link from "next/link";

export const Sidebar = async () => {
  return (
    <aside className="fixed inset-y-0 left-0 flex flex-col border-r w-40 items-center">
      <nav className="mt-2">
        <Button variant="ghost" asChild>
          <Link href="/dashboard/aktuality">
            <Pencil2Icon className="mr-2" /> Aktuality
          </Link>
        </Button>
      </nav>
    </aside>
  );
};

export default Sidebar;
