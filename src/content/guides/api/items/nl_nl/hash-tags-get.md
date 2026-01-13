---
[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Deze API gebruikt paginering, geleverd door de queryparameter `page`. HashTags worden geretourneerd in pagina's van `100`, gesorteerd op `tag`.

[inline-code-attrs-start title = 'HashTag cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'HashTag Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** De op te halen pagina, beginnend bij 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'HashTag Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Wordt opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Wordt opgenomen bij mislukking. **/
    reason?: string
    /** De hashtags! **/
    hashTags: HashTag[]
}
[inline-code-end]

---