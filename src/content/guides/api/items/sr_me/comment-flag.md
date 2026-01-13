[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Ова API крајња тачка омогућава означавање коментара за одређеног корисника.

Напомене:

- Овај позив увијек мора бити извршен у контексту корисника. Корисник може бити FastComments.com корисник, SSO корисник, или корисник тенанта.
- Ако је постављен праг за скривање на основу броја означавања, коментар ће бити аутоматски скривен уживо након што буде означен наведени број пута.
- Након аутоматског поништавања одобрења (скривања) — коментар може поново одобрити само администратор или модератор. Уклањање означавања неће поново одобрити коментар.

[inline-code-attrs-start title = 'Примјер cURL захтјева за означавање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

За анонимно означавање морамо навести `anonUserId`. Ово може бити ИД који представља анонимну сесију, или насумични UUID.
Ово нам омогућава да подржимо означавање и уклањање означавања коментара чак и ако корисник није пријављен. На тај начин, коментар може бити означен као
прозначен када се коментари преузму са истим `anonUserId`.

[inline-code-attrs-start title = 'Примјер cURL захтјева за означавање анонимног коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура захтјева за означавање коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за означавање коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    /** Да ли је коментар поништен (скривен) због превеликог броја означавања? **/
    wasUnapproved?: boolean;
}
[inline-code-end]

---