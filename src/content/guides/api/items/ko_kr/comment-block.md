[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 특정 댓글을 작성한 사용자를 차단할 수 있는 기능을 제공합니다. FastComments.com 사용자, SSO 사용자, Tenant 사용자가 작성한 댓글로부터의 차단을 지원합니다.

이 호출은 `commentIdsToCheck` 바디 파라미터를 지원하여, 이 작업 수행 후 클라이언트에 표시될 수 있는 다른 댓글들이 차단/차단 해제되어야 하는지 확인할 수 있습니다.

Notes:

- 이 호출은 항상 사용자 컨텍스트에서 이루어져야 합니다. 사용자는 FastComments.com 사용자, SSO 사용자, 또는 Tenant 사용자일 수 있습니다.
- 요청의 `userId`는 차단을 *행하는 사용자*입니다. 예: `User A`가 `User B`를 차단하려고 합니다. `userId=User A`와 `User B`가 작성한 댓글의 id를 전달하세요.
- 완전히 익명인 댓글(사용자 id 없음, 이메일 없음)은 차단할 수 없으며 에러가 반환됩니다.

[inline-code-attrs-start title = '댓글 차단 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

익명 차단의 경우 `anonUserId`를 지정해야 합니다. 이는 익명 세션을 나타내는 ID이거나 임의의 UUID일 수 있습니다.
이를 통해 사용자가 로그인하지 않은 경우에도 동일한 `anonUserId`로 댓글을 가져와 댓글을 차단하는 것을 지원할 수 있습니다.

[inline-code-attrs-start title = '익명 댓글 차단 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '댓글 차단 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = '댓글 차단 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** commentIdsToCheck가 정의된 경우, 이 맵에서 true인 항목들도 차단됩니다. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---