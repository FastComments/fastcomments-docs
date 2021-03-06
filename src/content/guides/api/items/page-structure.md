A `Page` object represents the page that many comments may belong to. This relationship is defined by
`urlId`.

A `Page` stores information such as the page title, comment count, and `urlId`.

The structure for the Page object is as follows:

[inline-code-attrs-start title = 'Page Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string;
    urlId: string;
    url: string;
    title: string;
    createdAt: string;
    commentCount: number;
}
[inline-code-end]
