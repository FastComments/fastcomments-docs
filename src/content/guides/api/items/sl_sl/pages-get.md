[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Trenutno lahko pridobite le vse strani (ali posamezno stran preko `/by-url-id`) povezanih z vašim računom. Če želite bolj natančno iskanje, [kontaktirajte nas](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Primer cURL zahtevka za strani'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtevka za strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključeno ob napaki. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Koristen nasvet

API `Comment` zahteva `urlId`. Najprej lahko pokličete API `Pages`, da vidite, kako
izgledajo vrednosti `urlId`, ki so vam na voljo.