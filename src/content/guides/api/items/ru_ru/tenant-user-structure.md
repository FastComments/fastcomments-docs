`TenantUser` определяет `User`, которым управляет конкретный tenant. Их учётная запись полностью контролируется tenant'ом, с которым они связаны, и её можно обновить или удалить через [UI](https://fastcomments.com/auth/my-account/users) или API.

Tenant users могут быть администраторами с полными правами и доступом к `Tenant`, либо иметь ограниченные права для модерирования комментариев, доступа к API-ключам и т.д.

Структура объекта `TenantUser` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** Это для уведомлений. **/
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
    /** Ссылка, например, на блог комментатора. **/
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
    /** Получает ли пользователь запросы помощи от комментаторов? **/
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