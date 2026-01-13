[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за отблокиране на потребител, написал даден коментар. Поддържа отблокиране от коментари, написани от FastComments.com потребители, SSO потребители и Tenant потребители.

Поддържа параметър `commentIdsToCheck` в тялото, за да провери дали други потенциално видими коментари на клиента трябва да бъдат блокирани/отблокирани след извършване на това действие.

Забележки:

- Това извикване винаги трябва да се прави в контекста на потребител. Потребителят може да бъде FastComments.com потребител, SSO потребител или Tenant потребител.
- `userId` в заявката е потребителят, който *извършва отблокирането*. Например: `Потребител A` иска да отблокира `Потребител B`. Подайте `userId=Потребител A` и id на коментара, който `Потребител B` е написал.
- Напълно анонимни коментари (без потребителски id, без имейл) не могат да бъдат блокирани и ще бъде върната грешка.

[inline-code-attrs-start title = 'Пример за отблокиране на коментар с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Пример за анонимно отблокиране на коментар с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за отблокиране на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за отблокиране на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are still blocked. If false, you might want to un-hide the comments from the user so they don't have to refresh. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]
