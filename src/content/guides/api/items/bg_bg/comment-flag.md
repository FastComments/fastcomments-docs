[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за докладване на коментар за конкретен потребител.

Забележки:

- Това извикване винаги трябва да се прави в контекста на потребител. Потребителят може да бъде FastComments.com потребител, SSO потребител или Tenant потребител.
- Ако е зададен праг за докладване-скриване, коментарът ще бъде автоматично скрит на живо след като бъде докладван определен брой пъти.
- След като бъде автоматично неодобрен (скрит) - коментарът може да бъде повторно одобрен само от администратор или модератор. Премахването на докладването няма да одобри отново коментара.

[inline-code-attrs-start title = 'Пример за докладване на коментар с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

За анонимно докладване трябва да посочим `anonUserId`. Това може да бъде ID, който представлява анонимната сесия, или случаен UUID.
Това ни позволява да поддържаме докладване и премахване на докладване на коментари дори ако потребителят не е влязъл в системата. По този начин коментарът може да бъде маркиран като
докладван, когато коментарите се извличат със същия `anonUserId`.

[inline-code-attrs-start title = 'Пример за анонимно докладване на коментар с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура на заявката за докладване на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за докладване на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
    /** Was the comment un-approved (hidden) due to being flagged too many times? **/
    wasUnapproved?: boolean;
}
[inline-code-end]
