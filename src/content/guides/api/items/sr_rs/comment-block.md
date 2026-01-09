[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Овај API ендпоинт пружа могућност блокирања корисника који је написао одређени коментар. Подржава блокирање корисника који су написали коментаре као FastComments.com корисници, SSO корисници и корисници тенанта.

Подржава параметар у телу захтева `commentIdsToCheck` који служи да се провери да ли неки други потенцијално видљиви коментари на клијенту треба да буду блокирани/одблокирани након извршења ове радње.

Напомене:

- Овај позив увек мора бити изведен у контексту корисника. Корисник може бити FastComments.com корисник, SSO корисник или корисник тенанта.
- Поле `userId` у захтеву означава корисника који је *извршава блокирање*. На пример: `User A` жели да блокира `User B`. Проследите `userId=User A` и id коментара који је написао `User B`.
- Потпуно анонимни коментари (нема user id, нема email) не могу бити блокирани и враћа се грешка.

[inline-code-attrs-start title = 'Пример cURL захтева за блокирање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

За анонимно блокирање, морамо назначити `anonUserId`. Ово може бити ID који представља анонимну сесију, или случајни UUID.
Ово нам омогућава да подржимо блокирање коментара чак и ако корисник није пријављен тако што ћемо дохватити коментаре са истим `anonUserId`.

[inline-code-attrs-start title = 'Пример cURL захтева за анонимно блокирање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Ако је `commentIdsToCheck` дефинисан, улази у овој мапи са вредношћу true су такође блокирани. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]