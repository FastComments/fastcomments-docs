[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт позволяет снять флаг с комментария для конкретного пользователя.

Примечания:

- Этот вызов всегда должен выполняться в контексте пользователя. Пользователем может быть FastComments.com User, SSO User, или Tenant User.
- После того как комментарий автоматически помечается как неодобренный (скрыт) — комментарий может быть повторно одобрен только администратором или модератором. Снятие флага не приведёт к повторному одобрению комментария.

[inline-code-attrs-start title = 'Пример cURL запроса для снятия флага с комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Для анонимной пометки необходимо указать `anonUserId`. Это может быть идентификатор, представляющий анонимную сессию, или случайный UUID.

[inline-code-attrs-start title = 'Пример cURL запроса для анонимного снятия флага с комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура запроса для снятия флага с комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа для снятия флага с комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Включается при ошибке. **/
    reason?: string
}
[inline-code-end]