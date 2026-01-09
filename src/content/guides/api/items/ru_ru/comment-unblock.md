[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Этот конечный пункт API предоставляет возможность разблокировать пользователя, который написал данный комментарий. Поддерживается разблокировка комментариев, написанных пользователями FastComments.com, SSO-пользователями и пользователями арендатора.

Поддерживается параметр тела `commentIdsToCheck` для проверки, должны ли какие-либо другие потенциально видимые комментарии на клиенте быть заблокированы/разблокированы после выполнения этого действия.

Примечания:

- Вызов всегда должен выполняться в контексте пользователя. Пользователь может быть пользователем FastComments.com, SSO-пользователем или пользователем арендатора.
- Поле `userId` в запросе — это пользователь, который *выполняет разблокировку*. Например: `User A` хочет разблокировать `User B`. Передайте `userId=User A` и идентификатор комментария, который написал `User B`.
- Полностью анонимные комментарии (без user id, без email) не могут быть заблокированы — будет возвращена ошибка.

[inline-code-attrs-start title = 'Пример cURL запроса для разблокировки комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL запроса для разблокировки анонимного комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура ответа на разблокировку комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Включается при ошибке. **/
    reason?: string
    /** Если определён commentIdsToCheck, записи в этой карте со значением true остаются заблокированными. Если false, вы, возможно, захотите снова показать эти комментарии пользователю, чтобы ему не пришлось обновлять страницу. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---