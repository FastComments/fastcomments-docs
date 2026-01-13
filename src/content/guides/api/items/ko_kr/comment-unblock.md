[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 특정 댓글을 작성한 사용자의 차단을 해제할 수 있는 기능을 제공합니다. FastComments.com 사용자, SSO 사용자, 및 테넌트 사용자가 작성한 댓글에서의 차단 해제를 지원합니다.

이 작업이 수행된 후 클라이언트에서 잠재적으로 표시되는 다른 댓글들이 차단/차단 해제되어야 하는지 확인하기 위해 `commentIdsToCheck` 바디 매개변수를 지원합니다.

참고:

- 이 호출은 항상 사용자의 맥락에서 이루어져야 합니다. 사용자는 FastComments.com 사용자, SSO 사용자, 또는 테넌트 사용자일 수 있습니다.
- 요청의 `userId`는 차단을 해제하는 *사용자*입니다. 예를 들어: `User A`가 `User B`의 차단을 해제하려고 합니다. `userId=User A`와 `User B`가 작성한 댓글 ID를 전달하세요.
- 완전히 익명인 댓글(사용자 id 없음, 이메일 없음)은 차단할 수 없으며 오류가 반환됩니다.

[inline-code-attrs-start title = '댓글 차단 해제 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '익명 댓글 차단 해제 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '댓글 차단 해제 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = '댓글 차단 해제 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** commentIdsToCheck가 정의되어 있으면, 이 맵에서 true인 항목들은 여전히 차단된 상태입니다. false인 경우에는 사용자가 새로고침할 필요가 없도록 댓글의 숨김을 해제해 줄 수 있습니다. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]