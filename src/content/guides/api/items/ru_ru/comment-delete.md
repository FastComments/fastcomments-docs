[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Этот конечный пункт API предоставляет возможность удаления комментария.

Примечания:

- Этот API может обновлять виджет комментариев "вживую", если необходимо (это увеличивает `creditsCost` с `1` до `2`).
- Этот API удалит все дочерние комментарии.
- Если целевой комментарий заблокирован (`isLocked: true`), запрос отклоняется с `code: 'locked'`. Сначала разблокируйте комментарий, затем удалите его.

[inline-code-attrs-start title = 'Пример cURL-запроса для удаления комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура запроса DELETE комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Пользователь, выполняющий обновление. При необходимости может использоваться для проверки того, может ли он удалить комментарий.  **/
    contextUserId?: string
	/** Нужно ли удалять комментарий "вживую" для пользователей, просматривающих экземпляры виджета комментариев с тем же urlId. ПРИМЕЧАНИЕ: Удваивает стоимость в кредитах с 1 до 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа DELETE комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Включено при ошибке. **/
    reason?: string
}
[inline-code-end]