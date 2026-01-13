[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 특정 사용자를 위해 댓글을 신고할 수 있는 기능을 제공합니다.

Notes:

- 이 호출은 항상 사용자 컨텍스트에서 이루어져야 합니다. 사용자는 FastComments.com 사용자, SSO 사용자, 또는 테넌트 사용자가 될 수 있습니다.
- 숨김 임계값(flag-to-hide threshold)이 설정되어 있으면, 댓글이 정의된 횟수만큼 신고되면 실시간으로 자동으로 숨김 처리됩니다.
- 댓글이 자동으로 미승인(숨김) 처리된 후에는 관리자나 모더레이터만이 댓글을 다시 승인할 수 있습니다. 플래그 해제는 댓글을 다시 승인하지 않습니다.

[inline-code-attrs-start title = '댓글 신고 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

For anonymous flagging, we must specify an `anonUserId`. This can be an ID that represents the anonymous session, or a random UUID.
This allows us to support flagging and un-flagging comments even if a user is not logged in. This way, the comment can be marked as
flagged when comments are fetched with the same `anonUserId`.

[inline-code-attrs-start title = '익명 댓글 신고 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = '댓글 신고 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = '댓글 신고 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 댓글이 너무 많이 신고되어 미승인(숨김) 처리되었나요? **/
    wasUnapproved?: boolean;
}
[inline-code-end]