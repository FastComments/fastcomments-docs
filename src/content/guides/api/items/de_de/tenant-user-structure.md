Der `TenantUser` definiert einen `User`, der von einem bestimmten Tenant verwaltet wird. Sein Konto steht vollständig unter der Kontrolle des Tenants,
mit dem er verknüpft ist, und sein Konto kann über die [Benutzeroberfläche](https://fastcomments.com/auth/my-account/users) oder API aktualisiert oder gelöscht werden.

Tenant-Benutzer können Administratoren mit allen Berechtigungen und Zugriff auf den `Tenant` sein, oder sie können auf bestimmte Berechtigungen beschränkt sein, um
Kommentare zu moderieren, auf API-Schlüssel zuzugreifen usw.

Die Struktur des `TenantUser`-Objekts ist wie folgt:

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
