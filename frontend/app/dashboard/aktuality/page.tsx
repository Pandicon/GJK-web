import { redirect } from "next/navigation";
import React from "react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/dashboard/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/dashboard/ui/table";
import { getArticles } from "@/lib/actions";
import { Button } from "@/components/dashboard/ui/button";
import { PlusCircledIcon } from "@radix-ui/react-icons";
import Link from "next/link";
import { getSession } from "@/lib/session";
import ArticleActions from "@/components/dashboard/article-actions";
import { UserPermission } from "@/lib/definitions";

const Aktuality = async () => {
  const session = await getSession();
  const articles = await getArticles(0);
  if (session && session.hasPermission(UserPermission.MANAGE_ARTICLES)) {
    return (
      <>
        <div className="flex justify-end">
          <Button className="mb-2" asChild>
            <Link href="/dashboard/nova-aktualita">
              <PlusCircledIcon className="mr-2" />
              Nová aktualita
            </Link>
          </Button>
        </div>
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
                    <TableCell>{a.author_name}</TableCell>
                    <TableCell>
                      <ArticleActions articleId={a.id} />
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
