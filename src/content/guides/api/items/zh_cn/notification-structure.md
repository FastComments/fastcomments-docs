一个 `Notification` 对象表示针对用户的通知。

`Notification` 对象是自动创建的，无法通过 API 创建。它们在一年后会过期。
通知不能被删除。但可以更新将 `viewed` 设为 `false`，并且可以按 `viewed` 查询。

用户也可以通过将通知中的 `optedOut` 设置为 `true` 来选择不接收针对特定评论的通知。将其设置为 `false` 可以重新选择接收。

存在不同的通知类型 —— 请检查 `relatedObjectType` 和 `type`。

通知的创建方式非常灵活，可由多种情景触发（参见 `NotificationType`）。

截至目前，`Notification` 的存在并不意味着会或应该发送电子邮件。相反，通知用于通知提要和相关集成。

`Notification` 对象的结构如下：

[inline-code-attrs-start title = 'Notification 对象结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** 如果有人回复你。 **/
    RepliedToMe = 0,
    /** 如果有人在你评论的线程中的任何位置回复（即使是嵌套的子评论）。 **/
    RepliedTransientChild = 1,
    /** 如果你的评论被点赞。 **/
    VotedMyComment = 2,
    /** 如果在你订阅的页面根评论处有新的评论。 **/
    SubscriptionReplyRoot = 3,
    /** 如果有人在你的个人资料上评论。 **/
    CommentedOnProfile = 4,
    /** 如果你有私信。 **/
    DirectMessage = 5,
    /** TrialLimits 仅适用于租户用户。 **/
    TrialLimits = 6,
    /** 如果你被 @提及。 **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** 使用 SSO 时，用户 id 的格式为 `<tenant id>:<user id>`。 **/
    userId?: string
    /** 在使用 SSO 时，你只需关心 `userId`。 **/
    anonUserId?: string
    /** urlId 几乎总是已定义。它仅在租户级通知（不常见）中为可选。 **/
    urlId?: string
    /** URL 被缓存以便快速导航到通知来源。 **/
    url?: string
    /** 页面标题被缓存以便快速查看通知来源。 **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** 例如，评论 id。 **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // 日期字符串
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName 和 fromUserAvatarSrc 在此处被缓存以便快速显示通知。用户更新时它们也会更新。 **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** 将此设置为 true 以停止接收该对象的通知。 **/
    optedOut?: boolean
}
[inline-code-end]