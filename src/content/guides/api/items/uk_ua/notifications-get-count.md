[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Цей маршрут повертає об'єкт, що містить кількість сповіщень у параметрі `count`.

Він повільніший за `/notification-count/` і коштує вдвічі більше кредитів, але дозволяє фільтрувати за більшою кількістю параметрів.

Ви можете фільтрувати за тими ж параметрами, що й кінцева точка `/notifications`, наприклад `userId`. При SSO ідентифікатор користувача має формат `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Приклад cURL: Кількість непрочитаних сповіщень для користувача'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Приклад cURL: Кількість непрочитаних сповіщень для користувача для конкретної сторінки'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту кількості сповіщень'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Фільтрувати за користувачем. **/
    userId?: string
    /** Фільтрувати за urlId. **/
    urlId?: string
    /** Фільтрувати за коментарем-джерелом. **/
    fromCommentId?: string
    /** Фільтрувати за прочитаністю. **/
    viewed?: 'true' | 'false'
    /** Фільтрувати за типом. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді кількості сповіщень'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Включено у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Включено у разі помилки. **/
    reason?: string
    count?: number
}
[inline-code-end]