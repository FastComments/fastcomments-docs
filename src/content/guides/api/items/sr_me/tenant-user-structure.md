`TenantUser` дефинише `User` којим управља одређени tenant. Њихов налог је у потпуној контроли тенанта са којим су повезани, и њихов налог може бити ажуриран или избрисан преко [UI](https://fastcomments.com/auth/my-account/users) или API-ја.

Tenant корисници могу бити администратори са свим дозволама и приступом `Tenant`-у, или могу бити ограничени на специфичне дозволе за модерирање коментара, приступ API кључевима итд.

Структура објекта `TenantUser` је следећа:

[inline-code-attrs-start title = 'Структура TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** Ово је за обавештења. **/
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
    /** Веза до блога коментатора, на пример. **/
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
    /** Да ли корисник прима захтеве за помоћ од коментатора? **/
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