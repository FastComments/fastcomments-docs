[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность заблокировать пользователя, который написал указанный комментарий. Поддерживается блокировка комментариев, написанных пользователями FastComments.com, SSO-пользователями и Tenant-пользователями.

Поддерживается параметр тела запроса `commentIdsToCheck`, который позволяет проверить, должны ли какие-либо другие потенциально видимые комментарии на клиенте быть заблокированы/разблокированы после выполнения этого действия.

Примечания:

- Этот вызов всегда должен выполняться в контексте пользователя. Пользователь может быть FastComments.com User, SSO User или Tenant User.
- `userId` в запросе — это пользователь, который *выполняет блокировку*. Например: `User A` хочет заблокировать `User B`. Передайте `userId=User A` и идентификатор комментария, который написал `User B`.
- Полностью анонимные комментарии (без user id и без email) не могут быть заблокированы, и будет возвращена ошибка.

[inline-code-attrs-start title = 'Пример cURL запроса для блокировки комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Для анонимной блокировки необходимо указать `anonUserId`. Это может быть идентификатор, представляющий анонимную сессию, или случайный UUID.
Это позволяет поддерживать блокировку комментариев даже если пользователь не вошёл в систему, получая комментарии с тем же `anonUserId`.

[inline-code-attrs-start title = 'Пример cURL запроса для анонимной блокировки комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса для блокировки комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Включено в случае ошибки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Включено в случае ошибки. **/
    reason?: string
    /** Если commentIdsToCheck определён, записи в этой карте со значением true также будут заблокированы. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---