 [api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

This route provides the ability to add a single authorized `Vote`. Votes can be `up` (+1) or `down` (-1).

[inline-code-attrs-start title = 'Vote Create cURL Example'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote Create Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Vote Create Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Creating Anonymous Votes

Anonymous votes can be created by setting `anonUserId` in the query params instead of `userId`.

This id does not have to correspond to a user object anywhere (hence anonymous). It is simply an identifier
for the session, so you can fetch votes again in the same session, to check if a comment has
been voted.

If you do not have such a thing as "anonymous sessions" like FastComments does - you can simply
set this to a random ID, like a UUID (although we appreciate smaller identifiers to save space).

### Other Notes

- This API obeys tenant-level settings. For example, if you disable voting for a given page, and you attempt to create a vote via the API, it will fail with error code `voting-disabled`.
- This API is live by default.
- This API will update the `votes` of the corresponding `Comment`.
