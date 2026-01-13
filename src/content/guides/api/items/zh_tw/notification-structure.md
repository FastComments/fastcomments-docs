A `Notification` object represents a notification for a user.

`Notification` objects are created automatically and cannot be created via the API. They also expire after one year.
Notifications cannot be deleted. They can however be updated to set `viewed` to `false`, and you can query by `viewed`.

A user may also opt out of notifications for a specific comment by setting `optedOut` in the notification to `true`. You can opt in again by setting it to `false`.

There are different notification types - check `relatedObjectType` and `type`.

The ways notifications are created is quite flexible and can be triggered by many scenarios (see `NotificationType`).

As of today, the existence of a `Notification` does not actually imply an email is or should be sent. Rather, the notifications
are used for the notification feed and related integrations.

The structure for the `Notification` object is as follows:

[inline-code-attrs-start title = '通知結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** 如果有人回覆了你。 **/
    RepliedToMe = 0,
    /** 如果有人在你評論之討論串中的任何地方回覆（即使是子留言的子留言）。 **/
    RepliedTransientChild = 1,
    /** 如果你的評論被按讚。 **/
    VotedMyComment = 2,
    /** 如果在你訂閱的頁面的根層有新評論。 **/
    SubscriptionReplyRoot = 3,
    /** 如果有人在你的個人資料上評論。 **/
    CommentedOnProfile = 4,
    /** 如果你有私人訊息。 **/
    DirectMessage = 5,
    /** TrialLimits 僅適用於租戶用戶。 **/
    TrialLimits = 6,
    /** 如果你被 @ 提及。 **/
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
    createdAt: string // 日期字串
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