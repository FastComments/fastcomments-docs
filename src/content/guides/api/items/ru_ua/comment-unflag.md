[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность снять флаг с комментария для конкретного пользователя.

Примечания:

- Этот вызов всегда должен выполняться в контексте пользователя. Пользователь может быть FastComments.com User, SSO User, или Tenant User.
- После того как комментарий автоматически становится неутверждённым (скрытым) — комментарий может быть повторно утверждён только администратором или модератором. Снятие флага не приведёт к повторному утверждению комментария.

[inline-code-attrs-start title = 'Пример cURL запроса: снятие флага с комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Для анонимного флагирования мы должны указать `anonUserId`. Это может быть идентификатор, представляющий анонимную сессию, или случайный UUID.

[inline-code-attrs-start title = 'Пример cURL запроса: анонимное снятие флага с комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура запроса снятия флага комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа при снятии флага с комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Указывается при ошибке. **/
    reason?: string
}
[inline-code-end]

---