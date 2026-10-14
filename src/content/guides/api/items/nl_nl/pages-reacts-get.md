[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Retourneert het aantal voor elke reactie op een pagina, en optioneel de gebruikers die gereageerd hebben, als één entry per reactie met zijn react‑id, meest recent eerst. Dit is de server‑side versie van [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` bepaalt welke gebruikers worden geretourneerd:

- `public` laat gebruikers weg die hun profielactiviteit privé hebben gemaakt.
- `include-private` neemt ze op. Gebruik dit alleen wanneer de lijst niet aan het publiek wordt getoond.

Anonieme gebruikers worden nooit opgenomen, en er worden maximaal 100 reacties opgezocht. Pagina's die niet bestaan retourneren geen `counts` en een lege `users` lijst.

[inline-code-attrs-start title = 'Voorbeeld cURL voor Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Page Reacts Request'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the counts. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Page Reacts Response'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Count per reaction id, for example {"heart": 12, "laugh": 3}. Not set when the page has no reactions. **/
    counts?: Record<string, number>
    /** One entry per react, most recent first. Only set when includeUserInfo is passed. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** The reaction id, for example "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Handige tip

Vergeet niet om waarden zoals de `urlId` URI‑gecodeerd te maken.

---