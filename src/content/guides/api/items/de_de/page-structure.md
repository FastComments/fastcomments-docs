Ein `Page`-Objekt repräsentiert die Seite, zu der viele Kommentare gehören können. Diese Beziehung wird durch
`urlId` definiert.

Eine `Page` speichert Informationen wie den Seitentitel, die Kommentaranzahl und die `urlId`.

Die Struktur des Page-Objekts ist wie folgt:

[inline-code-attrs-start title = 'Seiten Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
