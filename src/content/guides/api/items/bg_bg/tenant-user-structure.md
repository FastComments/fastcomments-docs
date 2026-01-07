`TenantUser` дефинира `User`, който се управлява от конкретен tenant. Акаунтът им е под пълен контрол на tenant-а,
с който са асоциирани, и техният акаунт може да бъде актуализиран или изтрит чрез [UI](https://fastcomments.com/auth/my-account/users) или API.

Tenant потребителите могат да бъдат администратори с всички права и достъп до `Tenant`, или могат да бъдат ограничени до конкретни права за
модериране на коментари, достъп до API ключове и т.н.

Структурата на обекта `TenantUser` е следната:

[inline-code-attrs-start title = 'Структура на TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** This is for notifications. **/
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
    /** A link to the commenter's blog, for example. **/
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
    /** Does the user receive help requests from commenters? **/
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
