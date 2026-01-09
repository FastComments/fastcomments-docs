[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Этот API использует пагинацию, задаваемую параметром запроса `page`. Хэштеги возвращаются страницами по `100`, отсортированы по `tag`.

[inline-code-attrs-start title = 'Пример cURL-запроса HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Страница для получения, начиная с 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включается при ошибке. **/
    reason?: string
    /** Хэштеги! **/
    hashTags: HashTag[]
}
[inline-code-end]

---