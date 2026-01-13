[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность пометить комментарий для конкретного пользователя.

Примечания:

- Этот вызов всегда должен выполняться в контексте пользователя. Пользователь может быть FastComments.com User, SSO User, или Tenant User.
- Если установлен порог flag-to-hide, комментарий будет автоматически скрыт в реальном времени после того, как он будет помечен указанное количество раз.
- После того как он автоматически станет неподтверждённым (скрытым) — комментарий может быть повторно одобрен только администратором или модератором. Снятие пометки не восстановит одобрение комментария.

[inline-code-attrs-start title = 'Пример cURL-запроса для пометки комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Для анонимной пометки необходимо указать `anonUserId`. Это может быть идентификатор, который представляет анонимную сессию, или случайный UUID.
Это позволяет поддерживать пометку и снятие пометки с комментариев даже если пользователь не вошёл в систему. Так комментарий может быть помечен как
flagged при получении комментариев с тем же `anonUserId`.

[inline-code-attrs-start title = 'Пример cURL-запроса для анонимной пометки комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура запроса пометки комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа пометки комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Включается при неудаче. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Включается при неудаче. **/
    reason?: string
    /** Комментарий был снят с одобрения (скрыт) из‑за слишком большого количества пометок? **/
    wasUnapproved?: boolean;
}
[inline-code-end]