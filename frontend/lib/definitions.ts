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
