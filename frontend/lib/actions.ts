"use server";
import { Article } from "./definitions";
import { getSession } from "./session";

export async function getArticles(page: number) {
  return await apiFetch(`/article/articles?page=${page}`, {
    cache: "no-cache", // TODO: Revalidate cache when an article is added, edited or deleted.
  }).then(async (res) => (await res.json()).articles as Array<Article>);
}

export async function postArticle(
  title: string,
  articleContent: string,
  tags: Array<string> = [],
) {
  const res = await authorizedApiFetch("/article/new", {
    body: JSON.stringify({
      title: title,
      content: articleContent,
      tags: tags,
      thumbnail_id: 0,
    }),
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (!res.ok) {
    throw new Error("Failed to post article: " + res.statusText);
  }
}

async function apiFetch(path: string, init?: RequestInit): Promise<Response> {
  return await fetch(`${process.env.API_HOST}${path}`, init);
}

async function authorizedApiFetch(
  path: string,
  init: RequestInit = {},
): Promise<Response> {
  const session = await getSession();
  const { headers, ...restRequestInit } = init;

  const updatedHeaders = new Headers(headers);
  updatedHeaders.append("Authorization", `Bearer ${session?.token}`);

  return await apiFetch(path, { headers: updatedHeaders, ...restRequestInit });
}
