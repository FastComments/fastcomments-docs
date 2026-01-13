The `TenantUser` definira `User` kojeg upravlja određeni tenant. Račun tog korisnika je u potpunoj kontroli tena kojem je pridružen te se račun može ažurirati ili izbrisati putem [UI](https://fastcomments.com/auth/my-account/users) ili API-ja.

Korisnici koje upravlja tenant mogu biti administratori s punim dopuštenjima i pristupom `Tenant`u, ili mogu imati ograničena dopuštenja za moderiranje komentara, pristup API ključevima itd.

The structure for the `TenantUser` object is as follows:

[inline-code-attrs-start title = 'Struktura objekta TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** Ovo je za obavijesti. **/
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
    /** Poveznica na blog komentatora, na primjer. **/
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
    /** Prima li korisnik zahtjeve za pomoć od komentatora? **/
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