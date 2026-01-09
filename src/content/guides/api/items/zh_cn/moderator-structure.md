`Moderator` 对象表示版主的配置。

有三种类型的版主：

1. 拥有 `isCommentModeratorAdmin` 标志的管理员用户。
2. 拥有 `isCommentModeratorAdmin` 标志的 SSO 用户。
3. 普通评论者，或被邀请为版主的 FastComments.com 用户。

`Moderator` 结构用于表示用例 `3` 的审核状态。

如果您想通过 API 邀请用户成为版主，请使用 `Moderator` API，创建一个 `Moderator` 并邀请他们。

如果用户没有 FastComments.com 帐户，邀请邮件会帮助他们完成设置。如果他们已有帐户，他们将
被授予对您租户的审核访问权限，并且 `Moderator` 对象的 `userId` 会更新为指向他们的用户。您将无法通过 API
访问他们的用户，因为在这种情况下该用户属于他们自己并由 FastComments.com 管理。

如果您需要完全管理该用户的帐户，我们建议使用 SSO，或将他们添加为 [Tenant User](https://fastcomments.com/auth/my-account/users) 并
然后添加一个 `Moderator` 对象来跟踪他们的统计信息。

`Moderator` 结构可作为用例 `1` 和 `2` 的统计跟踪机制。创建用户后，添加一个定义了其 `userId` 的 `Moderator`
对象，其统计信息将会在 [Comment Moderators Page](https://fastcomments.com/auth/my-account/moderate-comments/moderators) 上被跟踪。

`Moderator` 对象的结构如下：

[inline-code-attrs-start title = 'Moderator 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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