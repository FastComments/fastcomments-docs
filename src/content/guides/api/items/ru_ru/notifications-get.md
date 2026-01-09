[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Этот маршрут возвращает до 30 объектов `Notification`, отсортированных по `createdAt` — сначала самые новые.

Вы можете фильтровать по `userId`. При использовании SSO идентификатор пользователя имеет формат `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Непрочитанные уведомления для пользователя — пример cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса уведомлений'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Пагинация путем пропуска записей. **/
    skip?: number
    /** Фильтрация по пользователю. **/
    userId?: string
    /** Фильтрация по urlId. **/
    urlId?: string
    /** Фильтрация по исходному комментарию. **/
    fromCommentId?: string
    /** Фильтрация по прочитанности. **/
    viewed?: 'true' | 'false'
    /** Фильтрация по типу. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа уведомлений'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Указывается при ошибке. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]

---