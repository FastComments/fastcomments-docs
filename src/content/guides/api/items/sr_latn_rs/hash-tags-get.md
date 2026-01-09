[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Ovaj API koristi paginaciju, obezbeđenu parametrom upita `page`. Haštagovi se vraćaju u stranicama po `100`, poređani po `tag`.

[inline-code-attrs-start title = 'Primer cURL zahteva za HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Stranica koja se preuzima, počevši od 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    /** Haštagovi! **/
    hashTags: HashTag[]
}
[inline-code-end]

---