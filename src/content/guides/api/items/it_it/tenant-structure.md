Il `Tenant` definisce un cliente di FastComments.com. Possono essere creati tramite l'API dai tenant con accesso al white labeling. I tenant white-labeled non possono creare altri tenant white-labeled (è consentito un solo livello di annidamento).

La struttura dell'oggetto `Tenant` è la seguente:

[inline-code-attrs-start title = 'Struttura Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Questo può anche essere gestito tramite l'API DomainConfig. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // probabilmente vuoi Unknown
    logoSrc?: string, // percorso dell'immagine originale
    logoSrc100px?: string, // ridimensionata per le miniature
    footerUnsubscribeURL?: string,
    emailHeaders?: Record<string, string>,
    disableUnsubscribeLinks?: boolean,
    dkim?: {
        domainName: string,
        keySelector: string,
        privateKey: string
    }
}

export interface TenantBillingInfo {
    name: string
    address: string
    city: string
    state: string
    zip: string
    country: string
}

export enum TenantPaymentFrequency {
    Monthly = 0,
    Annually = 1
}

export interface Tenant {
    id: string
    name: string
    email?: string
    signUpDate: number; // number per motivi "legacy"
    packageId?: string | null
    paymentFrequency?: TenantPaymentFrequency
    billingInfoValid?: boolean
    billingHandledExternally?: boolean
    createdBy?: string
    isSetup?: boolean
    domainConfiguration: FastCommentsAPITenantDomainConfig[]
    billingInfo?: FastCommentsAPITenantBillingInfo
    stripeCustomerId?: string
    stripeSubscriptionId?: string
    stripePlanId?: string
    enableProfanityFilter?: boolean
    enableSpamFilter?: boolean
    lastBillingIssueReminderDate?: string
    removeUnverifiedComments?: boolean
    unverifiedCommentsTTLms?: number
    commentsRequireApproval?: boolean
    autoApproveCommentOnVerification?: boolean
    sendProfaneToSpam?: boolean
    /** @readonly - Calcolato in base a packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Calcolato in base a packageId. **/
    hasAuditing?: boolean
    /** Puoi memorizzare una coppia chiave-valore con il tenant che puoi usare per interrogare. Le chiavi non possono contenere "." o "$", o essere più lunghe di 100 caratteri. I valori non possono essere più lunghi di 2k caratteri. **/
    meta?: Record<string, string | null>
}
[inline-code-end]