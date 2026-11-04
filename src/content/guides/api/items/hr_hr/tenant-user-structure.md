`TenantUser` definira `User` kojim upravlja određeni tenant. Njihov račun ima potpunu kontrolu nad tenantom
s kojim su povezani, a njihov račun može se ažurirati ili izbrisati putem [UI-ja](https://fastcomments.com/auth/my-account/users) ili API-ja.

Tenant korisnici mogu biti administratori sa svim dozvolama i pristupom `Tenant`-u, ili mogu biti ograničeni na specifične dozvole
za moderiranje komentara, pristup API ključevima itd.

Struktura objekta `TenantUser` je sljedeća:

[inline-code-attrs-start title = 'Struktura TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Na primjer, poveznica na blog komentatora. **/
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