[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Synes om en side som den aktuelle bruger. Hver bruger kan synes om en side én gang: at synes om igen lykkes med koden `already-liked` og ændrer ikke tælleren.

Siden oprettes, hvis den endnu ikke findes. Send `title` for at sætte eller opdatere sidens titel.

[inline-code-attrs-start title = 'Side Like cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Side Like Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Angiver sidens titel. **/
    title?: string
    /** URI-kodet JSON af dit SSO-objekt. Udelad for anonyme brugere. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Side Like Svarets Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' når brugeren allerede har liket siden. Ellers inkluderet ved fejl. **/
    code?: 'already-liked' | string
    /** Inkluderet ved fejl. **/
    reason?: string
}
[inline-code-end]