[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Овај API endpoint пружа могућност блокирања корисника који је написао одређени коментар. Подржава блокирање коментара написаних од FastComments.com Users, SSO Users, и Tenant Users.

Подржава body параметар `commentIdsToCheck` који провјерава да ли би други потенцијално видљиви коментари на клијенту требало да буду блокирани/деблокирани након извршења ове акције.

Напомене:

- Овај позив мора увијек бити направљен у контексту корисника. Корисник може бити FastComments.com User, SSO User, или Tenant User.
- `userId` у захтјеву је корисник који *извршава блокирање*. На примјер: `User A` жели да блокира `User B`. Прослиједите `userId=User A` и id коментара који је написао `User B`.
- Потпуно анонимни коментари (није присутан user id, нема е-поште) не могу бити блокирани и биће враћена грешка.

[inline-code-attrs-start title = 'Примјер cURL захтјева за блокирање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

За анонимно блокирање, морамо навести `anonUserId`. Ово може бити id који представља анонимну сесију или насумични UUID.
Ово нам омогућава подршку блокирања коментара чак и ако корисник није пријављен тако што ћемо преузети коментаре са истим `anonUserId`.

[inline-code-attrs-start title = 'Примјер cURL захтјева за блокирање анонимног коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за блокирање коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за блокирање коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    /** Ако је commentIdsToCheck дефинисан, уноси у овој мапи који имају вриједност true ће такође бити блокирани. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---