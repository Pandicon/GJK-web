"use server";
import { revalidateTag } from "next/cache";
import { Article } from "./definitions";
import { getSession } from "./session";

export async function getArticles(page: number) {
  return await apiFetch(`/article/articles?page=${page}`, {
    cache: "no-cache", // TODO: Revalidate cache when an article is added, edited or deleted.
    next: {
      tags: ["article"],
    },
  }).then(async (res) => (await res.json()).articles as Array<Article>);
}

export async function postArticle(
  title: string,
  articleContent: string,
  thumbnail: ArrayBuffer,
  tags: Array<string> = [],
) {
  const thumbnailId = await newBlob(thumbnail);

  const res = await authorizedApiFetch("/article/new", {
    body: JSON.stringify({
      title: title,
      content: articleContent,
      tags: tags,
      thumbnail_id: thumbnailId,
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

export async function deleteArticle(id: number) {
  const res = await authorizedApiFetch(`/article/delete?id=${id}`, {
    method: "DELETE",
  });

  if (!res.ok) {
    throw Error("Failed to delete article: " + res.statusText);
  }

  revalidateTag("article");
}

async function apiFetch(path: string, init?: RequestInit): Promise<Response> {
  return await fetch(`${process.env.API_HOST}${path}`, init);
}

async function newBlob(data: ArrayBuffer) {
  return await authorizedApiFetch("/blob/new", {
    method: "POST",
    body: data,
  }).then(async (res) => (await res.json()).id as number);
}

async function authorizedApiFetch(
  path: string,
  init: RequestInit = {},
): Promise<Response> {
  const session = await getSession();
  const { headers, ...restRequestInit } = init;

  const updatedHeaders = new Headers(headers);
  updatedHeaders.append("Authorization", `Bearer ${session?.payload.token}`);

  return await apiFetch(path, { headers: updatedHeaders, ...restRequestInit });
}
