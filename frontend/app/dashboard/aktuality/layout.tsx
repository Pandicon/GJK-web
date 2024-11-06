import { UserPermission } from "@/lib/definitions";
import { getSession } from "@/lib/session";
import React from "react";
import { redirect } from "next/navigation";

const Layout = async ({ children }: { children: React.ReactNode }) => {
  const session = await getSession();
  if (
    session &&
    (await session.isValid()) &&
    session.hasPermission(UserPermission.MANAGE_ARTICLES)
  )
    return <>{children}</>;
  else {
    redirect("/dashboard");
  }
};

export default Layout;
