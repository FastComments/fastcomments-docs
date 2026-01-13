---
A `Moderator` object represents configuration for a moderator.

有三種類型的版主：

1. 擁有 `isCommentModeratorAdmin` 標記的管理員使用者。
2. 具有 `isCommentModeratorAdmin` 標記的 SSO 使用者。
3. 被邀請為版主的一般留言者，或 FastComments.com 的使用者。

`Moderator` 結構用於表示用例 `3` 的審核狀態。

如果您想透過 API 邀請某人成為版主，請使用 `Moderator` API，建立一個 `Moderator` 並邀請他們。

如果該使用者沒有 FastComments.com 帳號，邀請信將協助他們完成設定。如果他們已經有帳號，他們會被授予對您租戶的審核存取權，且 `Moderator` 物件的 `userId` 將更新為指向他們的使用者。您不會有該使用者的 API 存取權，因為在這種情況下該帳號屬於他們自己並由 FastComments.com 管理。

如果您需要完整管理該使用者的帳戶，我們建議使用 SSO，或將他們新增為 [租戶使用者](https://fastcomments.com/auth/my-account/users)，然後再新增一個 `Moderator` 物件以追蹤他們的統計資料。

`Moderator` 結構可以用作用例 `1` 和 `2` 的統計追蹤機制。建立使用者後，新增一個定義了他們 `userId` 的 `Moderator` 物件，他們的統計將會在 [留言版主頁面](https://fastcomments.com/auth/my-account/moderate-comments/moderators) 上被追蹤。

`Moderator` 物件的結構如下：

[inline-code-attrs-start title = 'Moderator 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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