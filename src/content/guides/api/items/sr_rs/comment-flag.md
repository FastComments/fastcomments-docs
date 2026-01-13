[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Овај API ендпоинт омогућава означавање коментара за одређеног корисника.

Напомене:

- Овај позив увек мора бити извршен у контексту корисника. Корисник може бити FastComments.com User, SSO User, или Tenant User.
- Ако је постављен праг за сакривање након пријављивања, коментар ће бити аутоматски сакривен уживо након што буде означен онај број пута који је дефинисан.
- Након што буде аутоматски поништено одобрење (скривен) — коментар може поново одобрити само администратор или модератор. Уклањање пријаве неће поново одобрити коментар.

[inline-code-attrs-start title = 'Пример cURL захтева за означавање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

За анонимно пријављивање, морамо навести `anonUserId`. Ово може бити ID који представља анонимну сесију, или случајни UUID.
Ово нам омогућава подршку за пријављивање и укидање пријаве коментара чак и ако корисник није пријављен. На овај начин, коментар може бити означен као пријављен када се коментари преузму са истим `anonUserId`.

[inline-code-attrs-start title = 'Пример cURL захтева за анонимно означавање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура захтева за означавање коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Да ли је коментар поништен (скривен) због превеликог броја пријава? **/
    wasUnapproved?: boolean;
}
[inline-code-end]

---