`TenantUser`, belirli bir kiracı (tenant) tarafından yönetilen bir `User` tanımlar. Hesapları, ilişkili oldukları kiracının tam kontrolündedir ve hesapları [UI](https://fastcomments.com/auth/my-account/users) veya API aracılığıyla güncellenebilir veya silinebilir.

Kiracı kullanıcıları, `Tenant` üzerinde tüm izinlere ve erişime sahip yöneticiler olabilir veya yalnızca yorumları moderasyon, API anahtarlarına erişim gibi belirli izinlerle sınırlı olabilirler.

`TenantUser` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'TenantUser Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** Bu bildirimler içindir. **/
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
    /** Örneğin, yorumcu bloguna bir bağlantı. **/
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
    /** Kullanıcı, yorumculardan yardım talepleri alıyor mu? **/
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