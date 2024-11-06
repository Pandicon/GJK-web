"server-only";
import { JWTPayload, SignJWT, jwtVerify } from "jose";
import { cookies } from "next/headers";
import { UserPermission } from "./definitions";

const secretKey = process.env.SESSION_SECRET;
const encodedKey = new TextEncoder().encode(secretKey);

async function encrypt(payload: SessionPayload) {
  return new SignJWT(payload)
    .setProtectedHeader({ alg: "HS256" })
    .setIssuedAt()
    .setExpirationTime("7d")
    .sign(encodedKey);
}

async function decrypt(jwt: string) {
  const jwtVerifyResult = await jwtVerify<SessionPayload>(jwt, encodedKey, {
    algorithms: ["HS256"],
  });
  return new Session(jwtVerifyResult.payload);
}

export async function createSession(token: string, perms: number) {
  const expiresAt = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000);
  const session: SessionPayload = {
    token: token,
    perms: perms,
    expiresAt: expiresAt,
  };
  const sessionToken = await encrypt(session);

  await setSessionCookie(sessionToken, expiresAt);
}

export async function getSession() {
  const jwt = (await cookies()).get("session")?.value;
  if (!jwt) return undefined;
  const session = await decrypt(jwt);
  return session;
}

export async function deleteSession() {
  (await cookies()).delete("session");
}

async function setSessionCookie(sessionToken: string, expiresAt: Date) {
  (await cookies()).set("session", sessionToken, {
    httpOnly: true,
    secure: true,
    expires: expiresAt,
    sameSite: "lax",
    path: "/",
  });
}

export type SessionPayload = {
  token: string;
  perms: number;
  expiresAt: Date;
};

class Session implements JWTPayload {
  [propName: string]: unknown;
  iss?: string | undefined;
  sub?: string | undefined;
  aud?: string | string[] | undefined;
  jti?: string | undefined;
  nbf?: number | undefined;
  exp?: number | undefined;
  iat?: number | undefined;

  payload: SessionPayload;

  constructor(payload: SessionPayload) {
    this.payload = payload;
  }

  hasPermission(permission: UserPermission): boolean {
    return (this.payload.perms & permission) == permission;
  }

  async isValid() {
    const res = await fetch(`${process.env.API_HOST}/auth/me`, {
      headers: {
        Authorization: `Bearer ${this.payload.token}`,
      },
    });

    if (res.status == 400) {
      return false;
    }
    return true;
  }
}
