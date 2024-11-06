import { JWTPayload } from "jose";

export type Article = {
  id: number;
  create_timestamp: number;
  title: string;
  author_name: string;
  content: string;
  tags: Array<string>;
  thumbnail_id: number;
};

export enum UserPermission {
  NONE = 0,
  READ_SUBSTITUTIONS = 1,
  READ_SHEDULES = 2,
  MANAGE_USERS = 4,
  MANAGE_ARTICLES = 8,
  READ_CALENDAR = 16,
}

export type SessionData = {
  token: string;
  perms: number;
  expiresAt: Date;
};

export class Session implements JWTPayload {
  data: SessionData;

  constructor(data: SessionData) {
    this.data = data;
  }

  hasPermission(permission: UserPermission): boolean {
    return (this.data.perms & permission) == permission;
  }

  async isValid(): Promise<boolean> {
    const res = await fetch(`${process.env.API_HOST}/auth/me`, {
      headers: {
        Authorization: `Bearer ${this.data.token}`,
      },
    });

    if (res.status == 400) {
      return false;
    }
    return true;
  }

  [propName: string]: unknown;
  iss?: string | undefined;
  sub?: string | undefined;
  aud?: string | string[] | undefined;
  jti?: string | undefined;
  nbf?: number | undefined;
  exp?: number | undefined;
  iat?: number | undefined;
}
