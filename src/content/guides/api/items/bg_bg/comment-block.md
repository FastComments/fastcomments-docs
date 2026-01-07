[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за блокиране на потребител, написал даден коментар. Поддържа блокиране от коментари, написани от FastComments.com потребители, SSO потребители и Tenant потребители.

Поддържа параметър `commentIdsToCheck` в тялото, за да провери дали други потенциално видими коментари на клиента трябва да бъдат блокирани/отблокирани след извършване на това действие.

Забележки:

- Това извикване винаги трябва да се прави в контекста на потребител. Потребителят може да бъде FastComments.com потребител, SSO потребител или Tenant потребител.
- `userId` в заявката е потребителят, който *извършва блокирането*. Например: `Потребител A` иска да блокира `Потребител B`. Подайте `userId=Потребител A` и id на коментара, който `Потребител B` е написал.
- Напълно анонимни коментари (без потребителски id, без имейл) не могат да бъдат блокирани и ще бъде върната грешка.

[inline-code-attrs-start title = 'Пример за блокиране на коментар с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

За анонимно блокиране трябва да посочим `anonUserId`. Това може да бъде ID, който представлява анонимната сесия, или случаен UUID.
Това ни позволява да поддържаме блокиране на коментари дори ако потребителят не е влязъл в системата, като извличаме коментарите със същия `anonUserId`.

[inline-code-attrs-start title = 'Пример за анонимно блокиране на коментар с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за блокиране на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за блокиране на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are also blocked. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]
