The `TenantUser` definiše `User` koji je pod upravljanjem konkretnog tenanta. Njihov nalog se u potpunosti nalazi pod kontrolom tenanta sa kojim su povezani, i njihov nalog može biti ažuriran ili obrisan putem [UI](https://fastcomments.com/auth/my-account/users) ili API-ja.

Tenant korisnici mogu biti administratori sa svim dozvolama i pristupom `Tenant`-u, ili mogu biti ograničeni na određene dozvole za moderiranje komentara, pristup API ključevima, itd.

Struktura za `TenantUser` objekat je sledeća:

[inline-code-attrs-start title = 'Struktura TenantUser objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** Ovo je za notifikacije. **/
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
    /** Na primer, link ka blogu komentatora. **/
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
    /** Da li korisnik prima zahteve za pomoć od komentatora? **/
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