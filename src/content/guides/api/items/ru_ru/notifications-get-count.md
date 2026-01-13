[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Этот маршрут возвращает объект, содержащий количество уведомлений в параметре `count`.

Он медленнее, чем `/notification-count/`, и требует вдвое больше кредитов, но позволяет фильтровать по большему числу параметров.

Вы можете фильтровать по тем же параметрам, что и у эндпоинта `/notifications`, например по `userId`. При SSO идентификатор пользователя имеет формат `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Пример cURL: количество непрочитанных уведомлений для пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL: количество непрочитанных уведомлений пользователя для конкретной страницы'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса количества уведомлений'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Фильтровать по пользователю. **/
    userId?: string
    /** Фильтровать по urlId. **/
    urlId?: string
    /** Фильтровать по исходному комментарию. **/
    fromCommentId?: string
    /** Фильтровать по прочитанности. **/
    viewed?: 'true' | 'false'
    /** Фильтровать по типу. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа количества уведомлений'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Присутствует при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Присутствует при ошибке. **/
    reason?: string
    count?: number
}
[inline-code-end]