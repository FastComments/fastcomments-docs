[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Этот эндпоинт API позволяет удалить комментарий.

Примечания:

- Этот API может обновлять виджет комментариев «в реальном времени», если требуется (это увеличивает `creditsCost` с `1` до `2`).
- Этот API удалит все дочерние комментарии.
- Если целевой комментарий заблокирован (`isLocked: true`), запрос отклоняется с `code: 'locked'`. Сначала разблокируйте комментарий, затем удалите.

[inline-code-attrs-start title = 'Пример cURL-запроса удаления комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура запроса на удаление комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Пользователь, выполняющий обновление. При необходимости можно использовать для проверки, может ли он удалить комментарий.  **/
    contextUserId?: string
	/** Будет ли комментарий удален "в реальном времени" для пользователей, просматривающих экземпляры виджета комментариев с тем же urlId. ПРИМЕЧАНИЕ: Удваивает стоимость в кредитах с 1 до 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа на удаление комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Указывается при ошибке. **/
    reason?: string
}
[inline-code-end]

---