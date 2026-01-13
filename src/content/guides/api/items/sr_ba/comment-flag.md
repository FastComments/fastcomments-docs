[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Овај API ендпоинт омогућава означавање коментара за одређеног корисника.

Notes:

- Овај позив увијек мора бити направљен у контексту корисника. Корисник може бити FastComments.com User, SSO User, или Tenant User.
- Ако је подешен праг за означавање који сакрива, коментар ће бити аутоматски скривен у реалном времену након што буде означен дефинисани број пута.
- Након што је аутоматски опозван (скривен) - коментар може поново одобрити само администратор или модератор. Уклањање ознаке неће поново одобрити коментар.

[inline-code-attrs-start title = 'Пример cURL захтева за означавање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

For anonymous flagging, we must specify an `anonUserId`. This can be an ID that represents the anonymous session, or a random UUID.
This allows us to support flagging and un-flagging comments even if a user is not logged in. This way, the comment can be marked as
flagged when comments are fetched with the same `anonUserId`.

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
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    /** Да ли је коментар аутоматски опозван (скривен) због превеликог броја ознака? **/
    wasUnapproved?: boolean;
}
[inline-code-end]

---