[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Овај API ендпоинт омогућава уклањање блокаде корисника који је написао одређени коментар. Подржава уклањање блокаде за коментаре написане од стране FastComments.com Users, SSO Users, и Tenant Users.

Подржава `commentIdsToCheck` параметар у тијелу захтјева за провјеру да ли неки други потенцијално видљиви коментари на клијенту требају бити блокирани/одблокирани након извршења ове акције.

Напомене:

- Овaј позив увијек мора бити направљен у контексту корисника. Корисник може бити FastComments.com User, SSO User, или Tenant User.
- `userId` у захтјеву је корисник који *уклања блокаду*. На примјер: `User A` жели да уклони блокаду `User B`. Прослиједите `userId=User A` и ид коментара који је `User B` написао.
- Потпуно анонимни коментари (без user id, без е-поште) не могу бити блокирани и вратиће се грешка.

[inline-code-attrs-start title = 'Примјер cURL захтјева за уклањање блокаде коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Примјер cURL захтјева за уклањање блокаде анонимног коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за уклањање блокаде коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за уклањање блокаде коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    /** Ако је commentIdsToCheck дефинисан, уноси у овој мапи који су true су и даље блокирани. Ако су false, можда ћете желети да поново прикажете коментаре кориснику да не мора да освежи страницу. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]