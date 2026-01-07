[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:comment-id'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at fjerne en enkelt kommentar.

[inline-code-attrs-start title = 'Comment Sletning cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Comment Sletning Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** The comment id to delete. Note this is in the path, not query params. **/
    commentId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment Sletning Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
