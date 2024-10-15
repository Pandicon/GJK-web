"use server";
import { Article } from "./definitions";
import { getSession } from "./session";

export async function getArticles(page: number) {
  const res = await fetch(
    `${process.env.API_HOST}/article/articles?page=${page}`,
    {
      cache: "no-cache", // TODO: Revalidate cache when an article is added, edited or deleted.
      headers: {
        "Content-Type": "application/json",
      },
    },
  );

  if (!res.ok) {
    throw new Error("Failed to fetch articles");
  }

  const json = await res.json();
  return json?.articles as Array<Article>;
}

export async function postArticle(
  title: string,
  articleContent: string,
  tags?: string[],
  thumbnail_id?: number,
) {
  const session = await getSession();

  const res = await fetch(`${process.env.API_HOST}/article/new`, {
    body: JSON.stringify({
      title: title,
      author: "Placeholder Author",
      content: articleContent,
      tags: tags ?? [],
      thumbnail_id: thumbnail_id ?? 0,
    }),
    method: "POST",
    headers: {
      Authorization: `Bearer ${session?.token}`,
      "Content-Type": "application/json",
    },
  });

  if (!res.ok) {
    throw new Error("Failed to post article: " + res.statusText);
  }
}
