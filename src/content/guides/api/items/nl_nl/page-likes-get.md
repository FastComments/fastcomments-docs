[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Retourneert het aantal likes op een pagina, en of de huidige gebruiker deze heeft geliket. Pagina's die nog niet bestaan, geven een `likeCount` van `0` terug.

[inline-code-attrs-start title = 'Page Likes cURL voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes aanvraagstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI-gecodeerde JSON van uw SSO-object. Laat weg voor anonieme gebruikers. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Inbegrepen bij een fout. **/
    code?: string
    /** Inbegrepen bij een fout. **/
    reason?: string
    likeCount: number
    /** Of de gebruiker die het verzoek doet de pagina heeft geliket. **/
    didLike: boolean
    /** Het aantal top-level reacties op de pagina. **/
    commentCount: number
    /** De id die wordt gebruikt om je te abonneren op live-updates voor deze pagina. **/
    urlIdWS: string
}
[inline-code-end]