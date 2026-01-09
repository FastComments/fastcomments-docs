[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Этот API использует пагинацию, задаваемую параметром запроса `page`. Хэштеги возвращаются страницами по `100`, отсортированные по `tag`.

[inline-code-attrs-start title = 'Пример запроса cURL для HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Страница для получения, начинается с 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** The hashtags! **/
    hashTags: HashTag[]
}
[inline-code-end]