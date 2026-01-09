[api-resource-header-start name = 'HashTag'; route = 'POST /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

Цей маршрут дозволяє додати один `HashTag`.

[inline-code-attrs-start title = 'Приклад cURL для створення HashTag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "tag": "#example",
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту для створення HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді для створення HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagPostResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-hash-tag' | 'tag-does-not-exist' | 'url-too-long' | 'invalid-tag'
    /** Включається у разі помилки. **/
    reason?: string
    hashTag?: HashTag; // Ми повертаємо повністю створений хештег у разі успіху.
}
[inline-code-end]

---