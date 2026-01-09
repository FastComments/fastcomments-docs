`TenantUser` definira `User`, ki ga upravlja določen tenant. Njihov račun je popolnoma pod nadzorom tenanta, s katerim so povezani, in njihov račun lahko tenant posodobi ali izbriše preko [UI](https://fastcomments.com/auth/my-account/users) ali API.

Uporabniki tenanta so lahko skrbniki z vsemi dovoljenji in dostopom do `Tenant`, ali pa so omejeni na določena dovoljenja za moderiranje komentarjev, dostop do API ključev itd.

Struktura za objekt `TenantUser` je naslednja:

[inline-code-attrs-start title = 'Struktura objekta TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** To je za obvestila. **/
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
    /** Na primer povezava do komentatorjevega bloga. **/
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
    /** Ali uporabnik prejema zahteve za pomoč od komentatorjev? **/
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