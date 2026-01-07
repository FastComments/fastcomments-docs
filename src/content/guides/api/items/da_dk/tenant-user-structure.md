`TenantUser` definerer en `User`, som administreres af en specifik tenant. Deres konto er under fuld kontrol af den tenant,
de er associeret med, og deres konto kan opdateres eller slettes via [UI'et](https://fastcomments.com/auth/my-account/users) eller API'et.

Tenant-brugere kan være administratorer med alle tilladelser og adgang til `Tenant`, eller de kan være begrænset til specifikke tilladelser til
at moderere kommentarer, tilgå API-nøgler osv.

Strukturen for `TenantUser`-objektet er som følger:

[inline-code-attrs-start title = 'TenantUser Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** This is for notifications. **/
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
    /** A link to the commenter's blog, for example. **/
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
    /** Does the user receive help requests from commenters? **/
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
