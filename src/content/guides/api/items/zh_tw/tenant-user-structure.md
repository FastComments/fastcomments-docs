`TenantUser` 定義了一個由特定租戶管理的 `User`。他們的帳戶完全由所屬的租戶控制，且可以透過 [UI](https://fastcomments.com/auth/my-account/users) 或 API 更新或刪除。

租戶使用者可以是擁有全部權限並可存取該 `Tenant` 的管理者，或僅限於特定權限以
管理留言、存取 API 金鑰等。

`The structure for the `TenantUser` object is as follows:`  (Wait, original had backticks around whole sentence? Actually original was: The structure for the `TenantUser` object is as follows:)

`TenantUser` 物件的結構如下：

[inline-code-attrs-start title = 'TenantUser 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** 這是用於通知。 **/
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
    /** 例如留言者的部落格連結。 **/
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
    /** 此使用者是否會接收來自留言者的求助請求？ **/
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