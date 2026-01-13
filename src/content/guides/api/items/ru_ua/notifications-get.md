[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Этот маршрут возвращает до 30 объектов `Notification`, отсортированных по `createdAt`, сначала самые новые.

Вы можете фильтровать по `userId`. При SSO идентификатор пользователя имеет формат `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Пример cURL запроса непрочитанных уведомлений для пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса уведомлений'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Пагинация путём пропуска записей. **/
    skip?: number
    /** Фильтр по пользователю. **/
    userId?: string
    /** Фильтр по urlId. **/
    urlId?: string
    /** Фильтр по исходному комментарию. **/
    fromCommentId?: string
    /** Фильтр по прочитанным/непрочитанным. **/
    viewed?: 'true' | 'false'
    /** Фильтр по типу. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа уведомлений'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Включается при ошибке. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]

---