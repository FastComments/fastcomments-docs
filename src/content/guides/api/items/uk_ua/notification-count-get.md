[api-resource-header-start name = 'NotificationCount'; route = 'GET /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

Цей маршрут повертає один `NotificationCount` за ідентифікатором користувача. З SSO ідентифікатор користувача має формат `<tenant id>:<user id>`.

Якщо немає непрочитаних сповіщень, `NotificationCount` не буде - отримаєте 404.

Це відрізняється від `notifications/count` тим, що є набагато швидшим, але не дозволяє фільтрування.

[inline-code-attrs-start title = 'Приклад cURL для NotificationCount за ID'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success","data":{"count":1,"createdAt":"2023-03-06T18:45:01.726Z","expireAt":"2024-03-06T01:25:01.726Z","id":"example"}}
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** Включається у разі помилки. **/
    reason?: string
    data?: NotificationCount
}
[inline-code-end]