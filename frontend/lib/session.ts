import "server-only";
import { JWTPayload, SignJWT, jwtVerify } from "jose";
import { cookies } from "next/headers";

const secretKey = process.env.SESSION_SECRET;
const encodedKey = new TextEncoder().encode(secretKey);

interface SessionData extends JWTPayload {
  token: string;
  perms: number;
  expiresAt: Date;
}

async function encrypt(payload: SessionData) {
  return new SignJWT(payload)
    .setProtectedHeader({ alg: "HS256" })
    .setIssuedAt()
    .setExpirationTime("7d")
    .sign(encodedKey);
}

async function decrypt(jwt: string) {
  const jwtVerifyResult = await jwtVerify<SessionData>(jwt, encodedKey, {
    algorithms: ["HS256"],
  });
  return jwtVerifyResult.payload as SessionData;
}

export async function createSession(token: string, perms: number) {
  const expiresAt = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000);
  const session: SessionData = {
    token: token,
    perms: perms,
    expiresAt: expiresAt,
  };
  const sessionToken = await encrypt(session);

  setSessionCookie(sessionToken, expiresAt);
}

export async function getSession() {
  const jwt = cookies().get("session")?.value;
  if (!jwt) return undefined;
  return await decrypt(jwt);
}

export function deleteSession() {
  cookies().delete("session");
}

function setSessionCookie(sessionToken: string, expiresAt: Date) {
  cookies().set("session", sessionToken, {
    httpOnly: true,
    secure: true,
    expires: expiresAt,
    sameSite: "lax",
    path: "/",
  });
}
