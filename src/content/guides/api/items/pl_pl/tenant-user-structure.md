`TenantUser` definiuje `User`, którym zarządza konkretny tenant. Ich konto jest w pełni kontrolowane przez tenant,
z którym są powiązani, a konto może być zaktualizowane lub usunięte za pomocą [UI](https://fastcomments.com/auth/my-account/users) lub API.

Użytkownicy tenantów mogą być administratorami z wszystkimi uprawnieniami i dostępem do `Tenant`, albo mogą mieć ograniczone do konkretnych uprawnień,
takich jak moderacja komentarzy, dostęp do kluczy API itp.

Struktura obiektu `TenantUser` jest następująca:

[inline-code-attrs-start title = 'Struktura TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** To dotyczy powiadomień. **/
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
    /** Na przykład link do bloga komentującego. **/
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
    /** Czy użytkownik otrzymuje prośby o pomoc od komentujących? **/
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