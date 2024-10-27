import Sidebar from "@/components/dashboard/sidebar";
import { Toaster } from "@/components/dashboard/ui/toaster";
import { getSession, isValidSession } from "@/lib/session";
import { redirect } from "next/navigation";

export default async function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await getSession();
  if (!session || !(await isValidSession(session))) {
    redirect("/login");
  } else {
    return (
      <>
        <Sidebar />
        <main className="ml-40 p-4">{children}</main>
        <Toaster />
      </>
    );
  }
}
