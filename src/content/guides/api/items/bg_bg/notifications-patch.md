[api-resource-header-start name = 'Notification'; route = 'PATCH /api/v1/notifications/:id'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за актуализиране на `Notification` по `id`.

Актуализирането на `Notification` има следните ограничения:

- Можете да актуализирате само следните полета:
  - `viewed`
  - `optedOut`

[inline-code-attrs-start title = 'Пример за PATCH на Notification с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/notifications/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"viewed": false,
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за PATCH на Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за PATCH на Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface NotificationPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
