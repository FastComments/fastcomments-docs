[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Returnerer antallet af likes på en side, og om den aktuelle bruger har liket den. Sider, der endnu ikke findes, returnerer en `likeCount` på `0`.

[inline-code-attrs-start title = 'Page Likes cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI-kodet JSON af dit SSO-objekt. Udelad for anonyme brugere. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Inkluderet ved fejl. **/
    code?: string
    /** Inkluderet ved fejl. **/
    reason?: string
    likeCount: number
    /** Om brugeren, der laver anmodningen, har liket siden. **/
    didLike: boolean
    /** Antallet af top‑niveau kommentarer på siden. **/
    commentCount: number
    /** Id'et, der bruges til at abonnere på live‑opdateringer for denne side. **/
    urlIdWS: string
}
[inline-code-end]