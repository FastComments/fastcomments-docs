[api-resource-header-start name = 'HashTag'; route = 'POST /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Этот маршрут позволяет добавить один `HashTag`.

[inline-code-attrs-start title = 'Пример cURL-запроса для создания HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "tag": "#example",
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса на создание HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа на создание HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostResponse {
    status: 'success' | 'failed'
    /** Включено при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag'
    /** Включено при ошибке. **/
    reason?: string
    hashTag?: HashTag; // При успешном выполнении возвращается полностью созданный хештег.
}
[inline-code-end]