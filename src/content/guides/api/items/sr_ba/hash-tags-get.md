[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Овај API користи пагинацију, која се обезбјеђује параметром упита `page`. HashTags се враћају у страницама по `100`, уређени по `tag`.

[inline-code-attrs-start title = 'HashTag cURL примјер'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Страница за преузимање, почиње од 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    /** Хаштагови! **/
    hashTags: HashTag[]
}
[inline-code-end]

---