[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Этот API-эндпойнт позволяет разблокировать пользователя, который написал указанный комментарий. Поддерживается разблокировка комментариев, написанных пользователями FastComments.com, SSO-пользователями и пользователями арендатора.

Поддерживается параметр тела `commentIdsToCheck`, чтобы проверить, следует ли заблокировать/разблокировать какие-либо другие потенциально видимые комментарии на клиенте после выполнения этого действия.

Примечания:

- Этот вызов всегда должен выполняться в контексте пользователя. Пользователь может быть пользователем FastComments.com, SSO-пользователем или пользователем арендатора.
- Параметр `userId` в запросе — это пользователь, который выполняет разблокировку. Например: `User A` хочет разблокировать `User B`. Передайте `userId=User A` и идентификатор комментария, который написал `User B`.
- Полностью анонимные комментарии (без идентификатора пользователя и без e-mail) не могут быть заблокированы — будет возвращена ошибка.

[inline-code-attrs-start title = 'Пример cURL-запроса разблокировки комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL-запроса разблокировки анонимного комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса на разблокировку комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа при разблокировке комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Включается при ошибке. **/
    reason?: string
    /** Если определен commentIdsToCheck, записи в этой карте со значением true остаются заблокированными. Если false, возможно, стоит снова показать комментарии пользователю, чтобы ему не пришлось обновлять страницу. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]