import type { MarkdownHeading } from "astro";


export interface SearchBy {
    title: any;
    headings: MarkdownHeading[];
    url: string | undefined
}