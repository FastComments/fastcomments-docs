[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Retourneert het aantal likes op een pagina, en optioneel de gebruikers die het hebben geliket, meest recent eerst. Dit is de server‑side versie van [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` bepaalt welke gebruikers worden geretourneerd:

- `public` laat gebruikers weg die hun profielactiviteit privé hebben gemaakt.  
- `include-private` neemt ze op. Gebruik dit alleen wanneer de lijst niet aan het publiek wordt getoond.

Anonieme gebruikers worden nooit opgenomen, en er worden maximaal 100 likes opgezocht. Pagina's die niet bestaan, retourneren een `likeCount` van `0` en een lege `users`‑lijst.

[inline-code-attrs-start title = 'Voorbeeld cURL voor Pagina Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina Likes Verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina Likes Respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    likeCount?: number
    /** Most recent first. Only set when includeUserInfo is passed. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Handige tip

Vergeet niet om waarden zoals de `urlId` te URI‑encoderen.