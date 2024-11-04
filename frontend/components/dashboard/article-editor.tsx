"use client";

import { Button } from "@/components/dashboard/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/dashboard/ui/form";
import { Input } from "@/components/dashboard/ui/input";
import { zodResolver } from "@hookform/resolvers/zod";
import React from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { postArticle } from "@/lib/actions";
import { toast } from "@/hooks/use-toast";
import TextEditor from "@/components/dashboard/text-editor";

const MAX_FILE_SIZE = 6000000;

const formSchema = z.object({
  title: z.string().max(80),
  content: z.string(),
  thumbnail: z.instanceof(File).refine((file) => file.size <= MAX_FILE_SIZE),
});

const ArticleEditor = () => {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <FormField
          control={form.control}
          name="title"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Titul</FormLabel>
              <FormControl>
                <Input {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="thumbnail"
          render={({ field: { value, onChange, ...fieldProps } }) => (
            <FormItem>
              <FormLabel>Náhledový obrázek</FormLabel>
              <FormControl>
                <Input
                  accept="image/*"
                  type="file"
                  onChange={(event) =>
                    onChange(event.target.files && event.target.files[0])
                  }
                  {...fieldProps}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="content"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Obsah</FormLabel>
              <FormControl>
                <TextEditor {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit">Zveřejnit</Button>
      </form>
    </Form>
  );

  async function onSubmit(values: z.infer<typeof formSchema>) {
    try {
      const thumbnailBuf = await values.thumbnail.arrayBuffer();
      await postArticle(values.title, values.content, thumbnailBuf);
      toast({ description: "Aktualita zveřejněna" });
    } catch (error: unknown) {
      toast({
        title: "Problém při zveřejňování aktuality",
        variant: "destructive",
      });
    }
  }
};

export default ArticleEditor;
