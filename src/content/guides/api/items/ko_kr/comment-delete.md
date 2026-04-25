[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 댓글을 삭제할 수 있는 기능을 제공합니다.

참고:

- 원하는 경우 이 API는 댓글 위젯을 "라이브"로 업데이트할 수 있습니다 (이 경우 `creditsCost`가 `1`에서 `2`로 증가합니다).
- 이 API는 모든 자식 댓글을 삭제합니다.
- 대상 댓글이 잠금 상태(`isLocked: true`)이면 요청은 `code: 'locked'`로 거부됩니다. 먼저 댓글의 잠금을 해제한 후 삭제하세요.

[inline-code-attrs-start title = '댓글 삭제 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = '댓글 삭제 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** 업데이트를 수행하는 사용자입니다. 원하는 경우 그 사용자가 댓글을 삭제할 수 있는지 확인하는 데 사용할 수 있습니다.  **/
    contextUserId?: string
	/** 동일한 urlId를 가진 댓글 위젯 인스턴스를 보고 있는 사용자들에게 댓글을 "라이브"로 삭제할지 여부입니다. 참고: 크레딧 비용이 1에서 2로 두 배가 됩니다. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '댓글 삭제 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]