The `TenantUser` визначає `User`, яким управляє конкретний тенант. Їхній обліковий запис повністю контролюється тим тенантом, з яким він пов'язаний, і обліковий запис може бути оновлений або видалений через [UI](https://fastcomments.com/auth/my-account/users) або API.

Користувачі тенанта можуть бути адміністраторами з усіма правами та доступом до `Tenant`, або їм можуть бути надані обмежені права для модерування коментарів, доступу до ключів API тощо.

Структура об'єкта `TenantUser` наведена нижче:

[inline-code-attrs-start title = 'Структура TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** Це для сповіщень. **/
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
    /** Наприклад, посилання на блог автора коментаря. **/
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
    /** Чи отримує користувач запити про допомогу від коментаторів? **/
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