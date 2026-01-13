[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

To API używa paginacji, udostępnianej przez parametr zapytania `page`. HashTags są zwracane w stronach po `100`, posortowane według `tag`.

[inline-code-attrs-start title = 'Przykład żądania cURL dla HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Strona do pobrania, zaczynając od 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    /** Hashtagi! **/
    hashTags: HashTag[]
}
[inline-code-end]

---