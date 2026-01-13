`Moderator` 객체는 모더레이터에 대한 구성을 나타냅니다.

모더레이터에는 세 가지 유형이 있습니다:

1. `isCommentModeratorAdmin` 플래그를 가진 관리자 사용자.
2. `isCommentModeratorAdmin` 플래그를 가진 SSO 사용자.
3. 초대로 모더레이터가 된 일반 댓글 작성자 또는 FastComments.com 사용자.

`Moderator` 구조는 사용 사례 `3`의 중재 상태를 나타내는 데 사용됩니다.

API를 통해 사용자를 모더레이터로 초대하려면 `Moderator` API를 사용하여 `Moderator`를 생성하고 그들을 `inviting` 하십시오.

사용자에게 FastComments.com 계정이 없으면 초대 이메일이 계정 설정을 돕습니다. 이미 계정이 있는 경우 해당 테넌트에 대한 중재 접근 권한이 부여되고 `Moderator` 객체의 `userId`가 해당 사용자를 가리키도록 업데이트됩니다. 이 경우 해당 사용자 계정은 본인 소유이며 FastComments.com에서 관리되므로, 해당 사용자에 대한 API 접근 권한은 없습니다.

사용자 계정을 완전히 관리해야 하는 경우, SSO를 사용하거나 그들을 [Tenant User](https://fastcomments.com/auth/my-account/users)로 추가한 다음 `Moderator` 객체를 추가하여 통계를 추적하는 것을 권장합니다.

`Moderator` 구조는 사용 사례 `1` 및 `2`에 대한 통계 추적 메커니즘으로 사용할 수 있습니다. 사용자를 생성한 후 `userId`가 정의된 `Moderator` 객체를 추가하면 해당 통계가 [Comment Moderators Page](https://fastcomments.com/auth/my-account/moderate-comments/moderators)에서 추적됩니다.

`Moderator` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'Moderator 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]

---