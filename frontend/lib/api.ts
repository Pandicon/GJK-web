import "server-only";
import { getSession } from "./session";

export async function apiFetch(path: string, init?: RequestInit) {
  return await fetch(`${process.env.API_HOST}${path}`, init);
}

export async function authorizedApiFetch(path: string, init: RequestInit = {}) {
  const session = await getSession();
  const { headers, ...restRequestInit } = init;

  const updatedHeaders = new Headers(headers);
  updatedHeaders.append("Authorization", `Bearer ${session?.payload.token}`);

  return await apiFetch(path, { headers: updatedHeaders, ...restRequestInit });
}
