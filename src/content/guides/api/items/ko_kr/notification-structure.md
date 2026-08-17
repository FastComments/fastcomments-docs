A `Notification` 객체는 사용자를 위한 알림을 나타냅니다.

`Notification` 객체는 자동으로 생성되며 API를 통해 생성할 수 없습니다. 또한 1년 후에 만료됩니다. 알림은 삭제할 수 없습니다. 하지만 `viewed`를 `false`로 설정하여 업데이트할 수 있으며, `viewed`로 조회할 수 있습니다.

사용자는 알림에서 특정 댓글에 대해 `optedOut`을 `true`로 설정하여 알림을 받지 않도록 선택할 수 있습니다. `false`로 설정하면 다시 수신하도록 할 수 있습니다.

알림 유형은 다양합니다 - `relatedObjectType`와 `type`을 확인하세요.

알림이 생성되는 방식은 매우 유연하며 다양한 시나리오에 의해 트리거될 수 있습니다 (`NotificationType` 참조).

현재 `Notification`이 존재한다고 해서 이메일이 전송되거나 전송되어야 함을 의미하지 않습니다. 대신 알림은 알림 피드와 관련 통합에 사용됩니다.

`Notification` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = '알림 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** If someone replied to you. **/
    RepliedToMe = 0,
    /** If someone replied anywhere in a thread (even children of children) of a thread you commented on. **/
    RepliedTransientChild = 1,
    /** If your comment was up-voted. **/
    VotedMyComment = 2,
    /** If a new comment is left on the root of a page you're subscribed to. **/
    SubscriptionReplyRoot = 3,
    /** If someone commented on your profile. **/
    CommentedOnProfile = 4,
    /** If you have a DM. **/
    DirectMessage = 5,
    /** TrialLimits is for tenant users only. **/
    TrialLimits = 6,
    /** If you were @mentioned. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId?: string
    /** When working with SSO, you only have to worry about `userId`. **/
    anonUserId?: string
    /** urlId is almost always defined. It is only optional for tenant-level notifications, which are infrequent. **/
    urlId?: string
    /** URL is cached for quick navigation to the source of the notification. **/
    url?: string
    /** Page Title is cached for quick reading of notification source. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** For example, comment id. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // date string
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName and fromUserAvatarSrc are cached here for quick displaying of the notification. They are updated when the user is updated. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Set this to true to stop getting notifications for this object. **/
    optedOut?: boolean
}
[inline-code-end]

---