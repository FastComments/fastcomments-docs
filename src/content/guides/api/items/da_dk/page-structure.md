Et `Page`-objekt repræsenterer den side, som mange kommentarer kan tilhøre. Dette forhold defineres af
`urlId`.

En `Page` gemmer information såsom sidetitlen, kommentarantal og `urlId`.

Strukturen for Page-objektet er som følger:

[inline-code-attrs-start title = 'Page Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Setting this to null means all SSO users can see the page. An empty list means it is closed to all users. **/
    accessibleByGroupIds?: string[] | null
    /** Is this page closed for new comments? **/
    isClosed?: boolean
}
[inline-code-end]
