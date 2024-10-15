import ArticleEditor from "@/components/dashboard/article-editor";
import { getSession, isValidSession } from "@/lib/session";
import { redirect } from "next/navigation";

const Page = () => {
  return <ArticleEditor />;
};

export default Page;
