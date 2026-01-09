[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Цей маршрут повертає до 30 об'єктів `Notification`, відсортованих за `createdAt`, від найновіших до найстаріших.

Ви можете фільтрувати за `userId`. При SSO ідентифікатор користувача має формат `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Приклад cURL: непрочитані сповіщення для користувача'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту Notifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Пагінація шляхом пропуску записів. **/
    skip?: number
    /** Фільтрувати за користувачем. **/
    userId?: string
    /** Фільтрувати за urlId. **/
    urlId?: string
    /** Фільтрувати за вихідним коментарем. **/
    fromCommentId?: string
    /** Фільтрувати за прочитаними/непрочитаними. **/
    viewed?: 'true' | 'false'
    /** Фільтрувати за типом. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Notifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Включено у випадку помилки. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]