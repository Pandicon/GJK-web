"use client";
import { Editor, EditorContent, useEditor } from "@tiptap/react";
import React from "react";
import Document from "@tiptap/extension-document";
import Paragraph from "@tiptap/extension-paragraph";
import Text from "@tiptap/extension-text";
import Bold from "@tiptap/extension-bold";
import Italic from "@tiptap/extension-italic";
import Link from "@tiptap/extension-link";
import { Toggle } from "@/components/dashboard/ui/toggle";
import { FontBoldIcon, FontItalicIcon } from "@radix-ui/react-icons";

const TextEditor = ({
  value,
  onChange,
}: {
  value: string;
  onChange: (value: string) => void;
}) => {
  const editor = useEditor({
    extensions: [Document, Paragraph, Text, Bold, Italic, Link],
    content: value,
    onUpdate: ({ editor }) => {
      onChange(editor.getHTML());
    },
    editorProps: {
      attributes: {
        class: "min-h-40 px-3 py-2 focus:outline-none",
      },
    },
    immediatelyRender: false,
  });
  return (
    <div className="border rounded-md">
      {editor ? <EditorToolbar editor={editor} /> : null}
      <EditorContent editor={editor} />
    </div>
  );
};

const EditorToolbar = ({ editor }: { editor: Editor }) => {
  return (
    <div className="flex gap-1 border-b p-1">
      <Toggle
        size="sm"
        pressed={editor.isActive("bold")}
        onPressedChange={() => editor.chain().focus().toggleBold().run()}
      >
        <FontBoldIcon />
      </Toggle>
      <Toggle
        size="sm"
        pressed={editor.isActive("italic")}
        onPressedChange={() => editor.chain().focus().toggleItalic().run()}
      >
        <FontItalicIcon />
      </Toggle>
      {/* TODO: Add link support */}
    </div>
  );
};

export default TextEditor;
