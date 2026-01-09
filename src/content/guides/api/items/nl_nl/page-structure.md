Een `Page`-object vertegenwoordigt de pagina waartoe veel opmerkingen kunnen behoren. Deze relatie wordt gedefinieerd door
`urlId`.

Een `Page` slaat informatie op zoals de paginatitel, het aantal opmerkingen en `urlId`.

De structuur voor het Page-object is als volgt:

[inline-code-attrs-start title = 'Pagina-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Als dit op null wordt gezet, betekent dit dat alle SSO-gebruikers de pagina kunnen zien. Een lege lijst betekent dat het gesloten is voor alle gebruikers. **/
    accessibleByGroupIds?: string[] | null
    /** Is deze pagina gesloten voor nieuwe opmerkingen? **/
    isClosed?: boolean
}
[inline-code-end]