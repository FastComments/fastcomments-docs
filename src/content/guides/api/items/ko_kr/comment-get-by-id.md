[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

이 API는 id로 단일 댓글을 가져오는 기능을 제공합니다.

[inline-code-attrs-start title = '댓글 ID 조회 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '댓글 ID 조회 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsGetByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '댓글 ID 조회 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentGetByIdResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'missing-id'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 댓글! **/
    comment?: Comment
}
[inline-code-end]

---