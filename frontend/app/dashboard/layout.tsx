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
import { getSession } from "@/lib/session";
import { redirect } from "next/navigation";
import { NewspaperIcon, HomeIcon } from "lucide-react";
import Link from "next/link";
import { UserPermission } from "@/lib/definitions";

export default async function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await getSession();
  if (session && (await session.isValid())) {
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
                    {session.hasPermission(UserPermission.MANAGE_ARTICLES) && (
                      <SidebarMenuItem>
                        <SidebarMenuButton asChild>
                          <Link href="/dashboard/aktuality">
                            <NewspaperIcon /> Aktuality
                          </Link>
                        </SidebarMenuButton>
                      </SidebarMenuItem>
                    )}
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
  } else {
    redirect("/login");
  }
}
