אובייקט `Page` מייצג את העמוד שתגובות רבות יכולות להשתייך אליו. יחס זה מוגדר על ידי
`urlId`.

`Page` מאחסן מידע כגון כותרת העמוד, מספר התגובות, ו-`urlId`.

המבנה עבור אובייקט Page הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
