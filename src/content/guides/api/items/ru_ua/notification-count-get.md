[api-resource-header-start name = 'NotificationCount'; route = 'GET /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

Этот маршрут возвращает один `NotificationCount` по идентификатору пользователя. При SSO идентификатор пользователя имеет формат `<tenant id>:<user id>`.

Если нет непрочитанных уведомлений, `NotificationCount` отсутствует — вы получите 404.

Это отличается от `notifications/count` тем, что намного быстрее, но не позволяет фильтровать.

[inline-code-attrs-start title = 'Пример cURL запроса NotificationCount по ID'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success","data":{"count":1,"createdAt":"2023-03-06T18:45:01.726Z","expireAt":"2024-03-06T01:25:01.726Z","id":"example"}}
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** Указывается при ошибке. **/
    reason?: string
    data?: NotificationCount
}
[inline-code-end]