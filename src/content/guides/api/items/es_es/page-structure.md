Un objeto `Page` representa la página a la que pueden pertenecer muchos comentarios. Esta relación se define por
`urlId`.

Una `Page` almacena información como el título de la página, el conteo de comentarios, y `urlId`.

La estructura del objeto Page es la siguiente:

[inline-code-attrs-start title = 'Estructura de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
