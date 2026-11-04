[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 특정 사용자를 위해 댓글을 신고(flag)하는 기능을 제공합니다.

참고 사항:

- 이 호출은 항상 사용자 컨텍스트에서 수행되어야 합니다. 사용자는 FastComments.com 사용자, SSO 사용자 또는 테넌트 사용자일 수 있습니다.
- 신고-숨김 임계값이 설정되어 있는 경우, 정의된 횟수만큼 신고되면 댓글이 실시간으로 자동 숨김 처리됩니다.
- 자동으로 미승인(숨김) 처리된 후에는 관리자 또는 중재자만 댓글을 다시 승인할 수 있습니다. 신고를 취소한다고 해서 댓글이 다시 승인되지는 않습니다.

[inline-code-attrs-start title = '댓글 신고 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

익명 신고의 경우 `anonUserId`를 지정해야 합니다. 이는 익명 세션을 나타내는 ID 또는 임의의 UUID일 수 있습니다.
이를 통해 사용자가 로그인하지 않은 경우에도 댓글 신고 및 신고 취소를 지원할 수 있습니다. 이렇게 하면 동일한 `anonUserId`로 댓글을 가져올 때 해당 댓글이 신고된 상태로 표시됩니다.

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
    /** 과도한 신고로 인해 댓글이 미승인(숨김) 처리되었습니까? **/
    wasUnapproved?: boolean;
}
[inline-code-end]