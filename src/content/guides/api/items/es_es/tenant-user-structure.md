El `TenantUser` define un `User` que es gestionado por un inquilino específico. Su cuenta está bajo control completo del inquilino
al que están asociados, y su cuenta puede ser actualizada o eliminada a través de la [UI](https://fastcomments.com/auth/my-account/users) o API.

Los usuarios de inquilino pueden ser administradores con todos los permisos y acceso al `Tenant`, o pueden estar limitados a permisos específicos para
moderar comentarios, acceder a claves de API, etc.

La estructura del objeto `TenantUser` es la siguiente:

[inline-code-attrs-start title = 'Estructura de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
