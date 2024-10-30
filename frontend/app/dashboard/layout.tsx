import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
} from "@/components/dashboard/ui/sidebar";
import { Toaster } from "@/components/dashboard/ui/toaster";
import { getSession, isValidSession } from "@/lib/session";
import { redirect } from "next/navigation";
import { NewspaperIcon, HomeIcon } from "lucide-react";
import Link from "next/link";

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
        <SidebarProvider>
          <Sidebar>
            <SidebarContent>
              <SidebarGroup>
                <SidebarGroupContent>
                  <SidebarMenu>
                    <SidebarMenuItem>
                      <SidebarMenuButton asChild>
                        <Link href="/dashboard/">
                          <HomeIcon /> Domů
                        </Link>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                    <SidebarMenuItem>
                      <SidebarMenuButton asChild>
                        <Link href="/dashboard/aktuality">
                          <NewspaperIcon /> Aktuality
                        </Link>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  </SidebarMenu>
                </SidebarGroupContent>
              </SidebarGroup>
            </SidebarContent>
          </Sidebar>
          <main className="p-6 w-full">{children}</main>
        </SidebarProvider>
        <Toaster />
      </>
    );
  }
}
