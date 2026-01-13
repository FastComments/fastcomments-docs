[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Je kunt momenteel alleen alle pagina's ophalen (of een enkele pagina via `/by-url-id`) die aan je account zijn gekoppeld. Als je fijnmaziger zoeken wilt, [neem contact met ons op](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Pages cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Pages Aanvraagstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Pages Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Opgenomen bij een fout. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Handige tip

De `Comment` API vereist een `urlId`. Je kunt eerst de `Pages` API aanroepen om te zien hoe de voor jou beschikbare `urlId`-waarden eruitzien.

---