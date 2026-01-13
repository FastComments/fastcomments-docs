[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

이 라우트는 id로 단일 SSO 사용자를 제거합니다.

이 사용자의 페이로드로 댓글 위젯을 다시 로드하면 사용자가 원활하게 재생성됩니다.

사용자의 댓글을 삭제하려면 `deleteComments` 쿼리 매개변수를 통해 가능합니다. 이 값이 true인 경우에 유의하세요:

1. 사용자의 모든 댓글이 실시간으로 삭제됩니다.
2. 모든 __child__ (이제 고아가 된) 댓글은 각 댓글의 연관된 페이지 설정에 따라 삭제되거나 익명화됩니다. 예를 들어 스레드 삭제 모드가 "anonymize"이면 답글은 남아 있고 사용자의 댓글만 익명화됩니다. 이는 `commentDeleteMode`가 `Remove`(기본값)일 때만 적용됩니다.
3. `creditsCost`는 `2`가 됩니다.

### 익명화된 댓글

사용자의 댓글을 유지하되 `commentDeleteMode=1`로 설정하여 간단히 익명화할 수 있습니다.

사용자의 댓글이 익명화되면 다음 값들이 null로 설정됩니다:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` 및 `isDeletedUser`는 `true`로 설정됩니다.

렌더링 시 댓글 위젯은 사용자의 이름에 `DELETED_USER_PLACEHOLDER` (기본값: "[deleted]")을, 댓글에는 `DELETED_CONTENT_PLACEHOLDER`를 사용합니다. 이는 Widget Customization UI에서 사용자화할 수 있습니다.

### 예제

[inline-code-attrs-start title = 'SSOUser 제거 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 제거 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // 기본값
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** 사용자의 댓글도 삭제하려면 이를 true로 설정할 수 있습니다. 이 경우 크레딧 비용이 두 배가 됩니다. **/
    deleteComments?: 'true' | 'false'
    /** 사용자의 댓글을 어떻게 처리할지 결정하려면 원하는 값으로 설정할 수 있습니다. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 제거 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** 실패 시 포함됩니다. **/
    reason?: string
    user?: SSOUser; // 성공 시 제거된 사용자를 반환합니다.
}
[inline-code-end]