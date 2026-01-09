`TenantUser` は特定のテナントによって管理される `User` を定義します。彼らのアカウントは関連するテナントによって完全に管理され、
彼らのアカウントは[UI](https://fastcomments.com/auth/my-account/users) または API を通じて更新または削除できます。

テナントユーザーは、すべての権限を持ち `Tenant` へのアクセスができる管理者になれます、または特定の権限に限定されて
コメントのモデレート、APIキーへのアクセスなどを行うことができます。

`TenantUser` オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'TenantUser の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** これは通知のためのものです。 **/
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
    /** 例えば、コメント投稿者のブログへのリンク。 **/
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
    /** このユーザーはコメント投稿者からのヘルプリクエストを受け取りますか？ **/
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