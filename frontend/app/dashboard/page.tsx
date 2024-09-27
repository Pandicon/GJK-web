import { verifySession } from "@/lib/dal";
import { redirect } from "next/navigation";
import React from "react";

const Dashboard = async () => {
  const session = await verifySession();
  if (session.isAuth) {
    return <div>Dashboard</div>;
  } else {
    redirect("/login");
  }
};

export default Dashboard;
