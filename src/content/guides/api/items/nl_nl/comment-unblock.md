[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Deze API-endpoint biedt de mogelijkheid om een gebruiker die een bepaalde reactie schreef te deblokkeren. Het ondersteunt het deblokkeren van reacties geschreven door FastComments.com Users, SSO Users, and Tenant Users.

Het ondersteunt een body-parameter `commentIdsToCheck` om te controleren of andere mogelijk zichtbare reacties in de client na deze actie geblokkeerd of gedeblokkeerd moeten worden.

Notes:

- Deze oproep moet altijd worden gedaan in de context van een gebruiker. De gebruiker kan een FastComments.com User, SSO User, or Tenant User zijn.
- The `userId` in the request is the user that is *die het deblokkeren uitvoert*. For example: `User A` wil `User B` deblokkeren. Pass `userId=User A` and the comment id that `User B` wrote.
- Completely anonymous comments (no user id, no email) cannot be blocked and an error will be returned.

[inline-code-attrs-start title = 'cURL-voorbeeld: het deblokkeren van een reactie'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'cURL-voorbeeld: het deblokkeren van een anonieme reactie'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van de aanvraag voor het deblokkeren van een reactie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van de response voor het deblokkeren van een reactie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Opgenomen bij mislukking. **/
    reason?: string
    /** Als commentIdsToCheck is gedefinieerd, zijn items in deze map met true nog steeds geblokkeerd. Als false, wilt u mogelijk de reacties voor de gebruiker weer zichtbaar maken zodat ze niet hoeven te verversen. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]