[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность заблокировать пользователя, который написал указанный комментарий. Поддерживается блокировка комментариев, написанных FastComments.com Users, SSO Users и Tenant Users.

Поддерживается параметр тела `commentIdsToCheck`, чтобы проверить, следует ли после выполнения этого действия заблокировать/разблокировать любые другие потенциально видимые на клиенте комментарии.

Примечания:

- Этот вызов всегда должен выполняться в контексте пользователя. Пользователь может быть FastComments.com User, SSO User или Tenant User.
- `userId` в запросе — это пользователь, который *выполняет блокировку*. Например: `User A` хочет заблокировать `User B`. Передайте `userId=User A` и идентификатор комментария, который написал `User B`.
- Полностью анонимные комментарии (без user id, без email) не могут быть заблокированы и будет возвращена ошибка.

[inline-code-attrs-start title = 'Пример cURL запроса блокировки комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Для анонимной блокировки необходимо указать `anonUserId`. Это может быть идентификатор, представляющий анонимную сессию, или случайный UUID.
Это позволяет поддерживать блокировку комментариев даже если пользователь не вошёл в систему, получая комментарии с тем же `anonUserId`.

[inline-code-attrs-start title = 'Пример cURL запроса блокировки анонимного комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса блокировки комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа при блокировке комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Указывается при ошибке. **/
    reason?: string
    /** Если определен commentIdsToCheck, элементы этой карты со значением true также считаются заблокированными. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---