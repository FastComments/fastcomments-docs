De `TenantUser` definieert een `User` die wordt beheerd door een specifieke tenant. Hun account staat volledig onder controle van de tenant waarmee ze zijn geassocieerd, en hun account kan worden bijgewerkt of verwijderd via de [UI](https://fastcomments.com/auth/my-account/users) of de API.

Tenantgebruikers kunnen beheerders zijn met alle rechten en toegang tot de `Tenant`, of ze kunnen beperkt zijn tot specifieke rechten om opmerkingen te modereren, toegang te krijgen tot API-sleutels, enz.

De structuur voor het `TenantUser`-object is als volgt:

[inline-code-attrs-start title = 'Structuur van TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** Dit is voor meldingen. **/
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
    /** Een link naar de blog van de reageerder, bijvoorbeeld. **/
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
    /** Ontvangt de gebruiker hulpverzoeken van reageerders? **/
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