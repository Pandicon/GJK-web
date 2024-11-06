import ArticleEditor from "@/components/dashboard/article-editor";
import { UserPermission } from "@/lib/definitions";
import { getSession } from "@/lib/session";
import { redirect } from "next/navigation";

const Page = async () => {
  const session = await getSession();
  if (session && session.hasPermission(UserPermission.MANAGE_ARTICLES))
    return <ArticleEditor />;
  else redirect("/dashboard");
};

export default Page;
