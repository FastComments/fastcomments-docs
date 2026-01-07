Le `TenantUser` définit un `User` qui est géré par un locataire spécifique. Leur compte est sous le contrôle complet du locataire
auquel ils sont associés, et leur compte peut être mis à jour ou supprimé via l'[interface utilisateur](https://fastcomments.com/auth/my-account/users) ou l'API.

Les utilisateurs locataires peuvent être des administrateurs avec toutes les permissions et accès au `Tenant`, ou ils peuvent être limités à des permissions spécifiques pour
modérer les commentaires, accéder aux clés API, etc.

La structure de l'objet `TenantUser` est la suivante :

[inline-code-attrs-start title = 'Structure de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
