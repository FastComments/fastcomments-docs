A `Notification` object represents a notification for a user.

`Notification` objects are created automatically and cannot be created via the API. They also expire after one year.
Notifications cannot be deleted. They can however be updated to set `viewed` to `false`, and you can query by `viewed`.

A user may also opt out of notifications for a specific comment by setting `optedOut` in the notification to `true`. You can opt in again by setting it to `false`.

There are different notification types - check `relatedObjectType` and `type`.

The ways notifications are created is quite flexible and can be triggered by many scenarios (see `NotificationType`).

As of today, the existence of a `Notification` does not actually imply an email is or should be sent. Rather, the notifications
are used for the notification feed and related integrations.

The structure for the `Notification` object is as follows:

[inline-code-attrs-start title = '알림 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** 누군가가 당신에게 답글을 남긴 경우. **/
    RepliedToMe = 0,
    /** 당신이 댓글을 남긴 스레드의 어느 곳(자식의 자식 등 포함)에서 누군가가 답글을 남긴 경우. **/
    RepliedTransientChild = 1,
    /** 당신의 댓글이 업보트된 경우. **/
    VotedMyComment = 2,
    /** 당신이 구독한 페이지의 루트에 새 댓글이 달린 경우. **/
    SubscriptionReplyRoot = 3,
    /** 누군가가 당신의 프로필에 댓글을 남긴 경우. **/
    CommentedOnProfile = 4,
    /** 당신에게 DM(다이렉트 메시지)이 도착한 경우. **/
    DirectMessage = 5,
    /** TrialLimits는 테넌트 사용자 전용입니다. **/
    TrialLimits = 6,
    /** 당신이 @언급된 경우. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId?: string
    /** SSO를 사용할 때에는 `userId`만 신경 쓰면 됩니다. **/
    anonUserId?: string
    /** urlId는 거의 항상 정의됩니다. 테넌트 수준의 알림에는 선택적이며, 이는 드뭅니다. **/
    urlId?: string
    /** URL은 알림 출처로 빠르게 이동하기 위해 캐시됩니다. **/
    url?: string
    /** 페이지 제목은 알림 출처를 빠르게 확인하기 위해 캐시됩니다. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** 예: 댓글 ID. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // 날짜 문자열
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName 및 fromUserAvatarSrc는 알림을 빠르게 표시하기 위해 여기에 캐시됩니다. 사용자가 업데이트되면 함께 업데이트됩니다. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** 이 값을 true로 설정하면 이 객체에 대한 알림 수신을 중단합니다. **/
    optedOut?: boolean
}
[inline-code-end]