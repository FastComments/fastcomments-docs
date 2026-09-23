[api-resource-header-start name = 'Poll'; route = 'DELETE /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

댓글에서 설문을 제거하고, 해당 설문에 대한 모든 투표도 함께 삭제합니다. 댓글 자체는 그대로 남습니다.

댓글을 삭제하면 설문과 투표도 함께 삭제되므로, 댓글을 유지하고 싶을 때만 이 작업이 필요합니다.

[inline-code-attrs-start title = 'Poll Delete cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Delete 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Delete 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]