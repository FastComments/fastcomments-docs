[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность пометить комментарий для конкретного пользователя.

Примечания:

- Этот вызов всегда должен выполняться в контексте пользователя. Пользователем может быть пользователь FastComments.com, SSO-пользователь или пользователь арендатора.
- Если установлен порог для скрытия по флагам, комментарий будет автоматически скрыт в реальном времени после того, как его пометят заданное количество раз.
- После того как он автоматически снят с одобрения (скрыт) - комментарий может быть повторно одобрен только администратором или модератором. Снятие флага не восстановит одобрение комментария.

[inline-code-attrs-start title = 'Пример cURL запроса пометки комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Для анонимной пометки мы должны указать `anonUserId`. Это может быть идентификатор, представляющий анонимную сессию, или случайный UUID.
Это позволяет поддерживать пометку и снятие пометки с комментариев, даже если пользователь не вошёл в систему. Таким образом комментарий может быть помечен как
flagged при получении комментариев с тем же `anonUserId`.

[inline-code-attrs-start title = 'Пример cURL запроса пометки анонимного комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Включено в случае ошибки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Включено в случае ошибки. **/
    reason?: string
    /** Был ли комментарий снят с одобрения (скрыт) из-за слишком большого числа жалоб? **/
    wasUnapproved?: boolean;
}
[inline-code-end]

---