The `TenantUser` дефинише `User` који је управљан одређеним tenant-ом. Налог је у потпуној контроли tenant-а са којим је повезан, и њихов налог може бити ажуриран или избрисан преко [UI](https://fastcomments.com/auth/my-account/users) или API-ја.

Tenant корисници могу бити администратори са свим дозволама и приступом `Tenant`-у, или им могу бити ограничене дозволе само за одређене радње као што су модерација коментара, приступ API кључевима, итд.

Структура за `TenantUser` објекат је следећа:

[inline-code-attrs-start title = 'Структура TenantUser објекта'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Веза до блога особе која је коментарисала, на пример. **/
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
    /** Да ли корисник прима захтјеве за помоћ од коментатора? **/
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