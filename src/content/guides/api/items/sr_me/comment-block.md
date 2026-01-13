[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Овај API endpoint пружа могућност блокирања корисника који је написао одређени коментар. Подржава блокирање на основу коментара које су написали FastComments.com корисници, SSO корисници и корисници тенанта.

Подржава body параметар `commentIdsToCheck` ради провере да ли неки други потенцијално видљиви коментари на клијенту треба да буду блокирани/отблокирани након извршене акције.

Напомене:

- Овај позив увек мора бити направљен у контексту корисника. Корисник може бити FastComments.com корисник, SSO корисник, или корисник тенанта.
- У захтеву, `userId` је корисник који *обавља блокирање*. На пример: `User A` жели да блокира `User B`. Проследите `userId=User A` и id коментара који је написао `User B`.
- Потпуно анонимни коментари (без user id-а, без е-поште) не могу бити блокирани и вратиће се грешка.

[inline-code-attrs-start title = 'Пример cURL захтева за блокирање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

За анонимно блокирање, морамо навести `anonUserId`. То може бити ID који представља анонимну сесију, или насумични UUID.
Ово нам омогућава да подржимо блокирање коментара чак и ако корисник није пријављен тако што ћемо преузети коментаре са истим `anonUserId`.

[inline-code-attrs-start title = 'Пример cURL захтева за блокирање анонимног коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за блокирање коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Ако је commentIdsToCheck дефинисан, уноси у овој мапи са вредношћу true су такође блокирани. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]