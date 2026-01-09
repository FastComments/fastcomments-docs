[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 특정 사용자가 댓글의 플래그를 해제할 수 있는 기능을 제공합니다.

참고:

- 이 호출은 항상 사용자 컨텍스트에서 이루어져야 합니다. 사용자는 FastComments.com 사용자, SSO 사용자, 또는 테넌트 사용자가 될 수 있습니다.
- 댓글이 자동으로 미승인(숨김)된 이후에는 해당 댓글을 다시 승인할 수 있는 사람은 관리자 또는 중재자뿐입니다. 플래그 해제는 댓글을 다시 승인하지 않습니다.

[inline-code-attrs-start title = '댓글 플래그 해제 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

익명 플래그의 경우 `anonUserId`를 지정해야 합니다. 이는 익명 세션을 나타내는 ID이거나 임의의 UUID일 수 있습니다.

[inline-code-attrs-start title = '익명 댓글 플래그 해제 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = '댓글 플래그 해제 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = '댓글 플래그 해제 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]

---