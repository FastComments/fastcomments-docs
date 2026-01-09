[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Ovaj API koristi paginaciju, koju osigurava query parametar `page`. HashTagovi se vraćaju po `100` po stranici, poredani po `tag`.

[inline-code-attrs-start title = 'HashTag cURL primjer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Stranica koju treba dohvatiti, počinje od 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** Hashtagovi! **/
    hashTags: HashTag[]
}
[inline-code-end]