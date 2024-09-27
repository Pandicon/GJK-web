import { createSession } from "@/lib/session";
import { redirect } from "next/navigation";
import { NextRequest, NextResponse } from "next/server";

export async function GET(request: NextRequest) {
  const params = request.nextUrl.searchParams;

  const res = await fetch(
    `http://localhost:2357/auth/oauth?${params.toString()}`,
  );

  if (!res.ok) {
    return new NextResponse("Failed to authenticate user: " + res.statusText);
  }

  const json = await res.json();
  const token = json.token;

  createSession(token);
  redirect("/dashboard");
}
