[api-resource-header-start name = 'HashTag'; route = 'DELETE /api/v1/hash-tags/:tag'; creditsCost = 1; api-resource-header-end]

Этот маршрут предоставляет удаление `HashTag` по указанному тегу.

Обратите внимание, что если автоматическое создание `HashTag` не отключено, хэштеги могут быть воссозданы пользователем, указавшим хэштег при комментировании.

[inline-code-attrs-start title = 'Пример cURL запроса для удаления HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/hash-tags/%23example_hash_tag?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса на удаление HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа на удаление HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagDeleteResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist'
    /** Включается при ошибке. **/
    reason?: string
}
[inline-code-end]