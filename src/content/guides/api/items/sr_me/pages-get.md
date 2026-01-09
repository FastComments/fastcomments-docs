[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Trenutno možete dohvatiti samo sve stranice (ili pojedinačnu stranicu putem `/by-url-id`) povezane sa vašim nalogom. Ako želite preciznije pretraživanje, [kontaktirajte nas](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Primjer cURL zahtjeva'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju greške. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Koristan savjet

The `Comment` API requires a `urlId`. You can call the `Pages` API first, to see what the `urlId` values available to you
look like.