import { getSession } from "@/lib/session";
import { redirect } from "next/navigation";
import React from "react";

const Dashboard = async () => {
  const session = await getSession();
  if (session) {
    return <div>Dashboard</div>;
  } else {
    redirect("/login");
  }
};

export default Dashboard;
