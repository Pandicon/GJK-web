import { redirect } from "next/navigation";
import React from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { getArticles } from "@/lib/actions";
import { Button } from "@/components/ui/button";
import { PlusCircledIcon } from "@radix-ui/react-icons";
import Link from "next/link";
import {
  DropdownMenu,
  DropdownMenuTrigger,
} from "@radix-ui/react-dropdown-menu";
import { DotsVerticalIcon } from "@radix-ui/react-icons";
import { getSession } from "@/lib/session";

const Aktuality = async () => {
  const session = await getSession();
  const articles = await getArticles(0);
  if (session && (session.perms & 8) == 8) {
    return (
      <>
        <Button className="mb-2" asChild>
          <Link href="/dashboard/nova-aktualita">
            <PlusCircledIcon className="mr-2" />
            Nová aktualita
          </Link>
        </Button>
        <Card>
          <CardHeader>
            <CardTitle>Aktuality</CardTitle>
          </CardHeader>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Datum vytvoření</TableHead>
                  <TableHead>Titul</TableHead>
                  <TableHead>Autor</TableHead>
                  <TableHead>
                    <span className="sr-only">Možnosti</span>
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {articles.map((a) => (
                  <TableRow key={a.id}>
                    <TableCell>
                      {new Date(a.create_timestamp * 1000).toLocaleDateString(
                        "cs-CZ",
                      )}
                    </TableCell>
                    <TableCell>{a.title}</TableCell>
                    <TableCell>{a.author}</TableCell>
                    <TableCell>
                      <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                          <Button variant="ghost" size="icon">
                            <DotsVerticalIcon />
                          </Button>
                        </DropdownMenuTrigger>
                      </DropdownMenu>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      </>
    );
  } else {
    redirect("/dashboard");
  }
};

export default Aktuality;
