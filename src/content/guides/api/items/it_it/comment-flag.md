[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

This API endpoint provides the ability to flag a comment for a specific user.

Notes:

- This call must always be made in the context of a user. The user can be a FastComments.com User, SSO User, or Tenant User.
- If a flag-to-hide threshold is set, the comment will be automatically hidden live after it has been flagged the defined number of times.
- After it is automatically un-approved (hidden) - the comment can only be re-approved by an administrator or moderator. Un-flagging will not re-approve the comment.

[inline-code-attrs-start title = 'Esempio cURL Segnalazione Commento'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

For anonymous flagging, we must specify an `anonUserId`. This can be an ID that represents the anonymous session, or a random UUID.
This allows us to support flagging and un-flagging comments even if a user is not logged in. This way, the comment can be marked as
flagged when comments are fetched with the same `anonUserId`.

[inline-code-attrs-start title = 'Esempio cURL Segnalazione Commento Anonimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struttura della richiesta per la segnalazione di un commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta per la segnalazione di un commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Incluso in caso di fallimento. **/
    reason?: string
    /** Il commento è stato non approvato (nascosto) a causa di troppe segnalazioni? **/
    wasUnapproved?: boolean;
}
[inline-code-end]