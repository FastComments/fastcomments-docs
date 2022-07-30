[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

This API endpoint provides the ability to un-block a user that wrote a given comment. It supports un-blocking from comments written by FastComments.com Users, SSO Users, and Tenant Users.

It supports a `commentIdsToCheck` body parameter to check if any other potentially visible comments on the client should be blocked/unblocked after this action is performed.

Notes:

- This call must always be made in the context of a user. The user can be a FastComments.com User, SSO User, or Tenant User.
- The `userId` in the request is the user that is *doing the un-blocking*. For example: `User A` wants to Un-Block `User B`. Pass `userId=User A` and the comment id that `User B` wrote.
- Completely anonymous comments (no user id, no email) cannot be blocked and an error will be returned.

[inline-code-attrs-start title = 'Comment Un-Block cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Comment Un-Block Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment Un-Block Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are still blocked. If false, you might want to un-hide the comments from the user so they don't have to refresh. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]
