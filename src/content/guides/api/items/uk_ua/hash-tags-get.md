[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Цей API використовує пагінацію, яку задає параметр запиту `page`. HashTags повертаються сторінками по `100`, впорядковані за `tag`.

[inline-code-attrs-start title = 'Приклад cURL для HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Номер сторінки для отримання, починаючи з 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** Включається у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включається уразі невдачі. **/
    reason?: string
    /** Хештеги! **/
    hashTags: HashTag[]
}
[inline-code-end]