`TenantUser` 定义了由特定租户管理的 `User`。他们的账户完全由所关联的租户掌控，且可以通过 [UI](https://fastcomments.com/auth/my-account/users) 或 API 更新或删除。

租户用户可以是具有对 `Tenant` 的全部权限和访问权的管理员，也可以被限制为仅拥有特定权限，例如管理评论、访问 API 密钥等。

`TenantUser` 对象的结构如下：

[inline-code-attrs-start title = 'TenantUser 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** 这是用于通知。 **/
export enum UserDigestEmailFrequency {
    Disabled = -1,
    Daily = 0,
    Weekly = 1,
    Monthly = 2
}

export interface TenantUser {
    id: string
    tenantId: string
    username: string
    /** 例如评论者博客的链接。 **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    verified: boolean
    loginCount: number
    optedInNotifications: boolean
    optedInTenantNotifications: boolean
    hideAccountCode: boolean
    avatarSrc?: string
    /** 用户是否接收来自评论者的帮助请求？ **/
    isHelpRequestAdmin: boolean
    isAccountOwner: boolean
    isAdminAdmin: boolean
    isBillingAdmin: boolean
    isAnalyticsAdmin: boolean
    isCustomizationAdmin: boolean
    isManageDataAdmin: boolean
    isCommentModeratorAdmin: boolean
    isAPIAdmin: boolean
    moderatorIds: string[]
    locale: FastCommentsLocale
    digestEmailFrequency: UserDigestEmailFrequency
    lastLoginDate: string
    displayLabel?: string
    karma?: number
}
[inline-code-end]

---