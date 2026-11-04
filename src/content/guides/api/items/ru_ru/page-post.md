[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность создавать страницы.

Распространённый сценарий использования — контроль доступа.

Примечания:

- Если вы оставляли комментарий в ветке комментариев или вызывали API для создания `Comment`, вы уже создали объект `Page`! Вы можете попробовать получить его через
  маршрут `Page` `/by-url-id`, передав тот же `urlId`, который был передан виджету комментариев.
- Структура `Page` содержит некоторые **вычисляемые** значения.
  В настоящее время это `commentCount` и `rootCommentCount`.
  Они заполняются автоматически и не могут быть заданы через API. Попытка сделать это приведёт к тому, что API вернёт ошибку.

[inline-code-attrs-start title = 'Пример cURL-запроса POST для Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура POST-запроса Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура POST-ответа Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Включается при ошибке. **/
    reason?: string
    /** Созданная страница. **/
    page?: Page
}
[inline-code-end]