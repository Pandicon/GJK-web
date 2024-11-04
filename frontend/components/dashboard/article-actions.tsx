"use client";

import React from "react";
import {
  DropdownMenu,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuContent,
} from "./ui/dropdown-menu";
import { DotsVerticalIcon } from "@radix-ui/react-icons";
import { Button } from "./ui/button";
import { deleteArticle } from "@/lib/actions";
import { toast } from "@/hooks/use-toast";

const ArticleActions = ({ articleId }: { articleId: number }) => {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="icon">
          <DotsVerticalIcon />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem
          onSelect={async () => {
            try {
              await deleteArticle(articleId);
              toast({ title: "Aktualita úspěšně smazána" });
            } catch {
              toast({
                title: "Problém při mazání aktuality",
                variant: "destructive",
              });
            }
          }}
        >
          Smazat
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};

export default ArticleActions;
