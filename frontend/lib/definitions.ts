export type Article = {
  id: number;
  create_timestamp: number;
  title: string;
  author_name: string;
  content: string;
  tags: Array<string>;
  thumbnail_id: number;
};
