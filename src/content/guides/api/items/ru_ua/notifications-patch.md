[api-resource-header-start name = 'Notification'; route = 'PATCH /api/v1/notifications/:id'; creditsCost = 1; api-resource-header-end]

Этот API-эндпойнт позволяет обновить `Notification` по `id`.

Обновление `Notification` имеет следующие ограничения:

- Вы можете обновлять только следующие поля:
  - `viewed`
  - `optedOut`

[inline-code-attrs-start title = 'Пример cURL-запроса PATCH для Notification'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/notifications/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"viewed": false,
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса PATCH для Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа PATCH для Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface NotificationPatchResponse {
    status: 'success' | 'failed'
    /** Присутствует при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Присутствует при ошибке. **/
    reason?: string
}
[inline-code-end]

---