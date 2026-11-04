[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

이 라우트는 id로 `TenantUser`를 삭제하는 기능을 제공합니다.

`deleteComments` 쿼리 파라미터를 통해 사용자의 댓글을 삭제할 수 있습니다. 이 값이 true인 경우 다음에 유의하세요:

1. 사용자의 모든 댓글이 실시간으로 삭제됩니다.
2. 모든 __자식__ (이제 고아가 된) 댓글은 각 댓글에 연결된 페이지 설정에 따라 삭제되거나 익명화됩니다. 예를 들어 스레드 삭제 모드가 "anonymize"인 경우 답글은 남아 있고 사용자의 댓글은 익명화됩니다. 이는 `commentDeleteMode`가 `Remove`(기본값)일 때만 적용됩니다.
3. `creditsCost`가 `2`가 됩니다.

### 익명화된 댓글

`commentDeleteMode=1`로 설정하면 사용자의 댓글을 유지하면서 단순히 익명화할 수 있습니다.

사용자의 댓글이 익명화되면 다음 값들이 null로 설정됩니다:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted`와 `isDeletedUser`는 `true`로 설정됩니다.

렌더링 시 댓글 위젯은 사용자 이름에 `DELETED_USER_PLACEHOLDER`(기본값: "[deleted]")를, 댓글 내용에 `DELETED_CONTENT_PLACEHOLDER`를 사용합니다. 이는 위젯 커스터마이제이션 UI를 통해 사용자 정의할 수 있습니다.

### 예제

[inline-code-attrs-start title = 'TenantUser 삭제 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 삭제 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // 기본값
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** 사용자의 댓글도 함께 삭제하려면 이 값을 true로 설정할 수 있습니다. 크레딧 비용이 두 배가 됩니다. **/
    deleteComments?: 'true' | 'false'
    /** 사용자의 댓글을 처리하는 방법을 결정하려면 이 값을 원하는 대로 설정할 수 있습니다. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 삭제 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]