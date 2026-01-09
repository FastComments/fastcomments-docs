---
Il `TenantUser` definisce un `User` che è gestito da un tenant specifico. Il loro account è sotto il completo controllo del tenant
a cui sono associati, e il loro account può essere aggiornato o eliminato tramite l'[UI](https://fastcomments.com/auth/my-account/users) o l'API.

Gli utenti del tenant possono essere amministratori con tutti i permessi e accesso al `Tenant`, oppure possono essere limitati a permessi specifici per
moderare i commenti, accedere alle chiavi API, ecc.

La struttura dell'oggetto `TenantUser` è la seguente:

[inline-code-attrs-start title = 'Struttura di TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** Questo è per le notifiche. **/
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
    /** Un link al blog del commentatore, ad esempio. **/
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
    /** L'utente riceve richieste di aiuto dai commentatori? **/
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