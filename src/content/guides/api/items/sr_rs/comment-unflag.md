[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Овај API ендпоинт пружа могућност уклањања ознаке (un-flag) коментара за одређеног корисника.

Напомене:

- Овaј захтев увек мора бити извршен у контексту корисника. Корисник може бити FastComments.com корисник, SSO корисник, или Tenant корисник.
- Након што је коментар аутоматски означен као неприхваћен (скривен) - коментар може бити поново одобрен само од стране администратора или модератора. Уклањање ознаке (un-flag) неће поново одобрити коментар.

[inline-code-attrs-start title = 'Пример cURL захтева за уклањање ознаке коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

За анонимно означавање, морамо навести `anonUserId`. То може бити ИД који представља анонимну сесију, или насумични UUID.

[inline-code-attrs-start title = 'Пример cURL захтева за анонимно означавање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура захтева за уклањање ознаке коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за уклањање ознаке коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---