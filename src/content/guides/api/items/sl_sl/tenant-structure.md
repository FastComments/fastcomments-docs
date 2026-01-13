`Tenant` predstavlja stranko FastComments.com. Ustvarijo jih lahko preko API-ja najemniki z dostopom za white-labeling. Najemniki z white-labelingom ne morejo ustvarjati drugih najemnikov z white-labelingom (dovoljena je le ena raven gnezdenja).

Struktura objekta `Tenant` je naslednja:

[inline-code-attrs-start title = 'Struktura objekta `Tenant`'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** To je mogoče tudi upravljati preko DomainConfig API-ja. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // verjetno želite Unknown
    logoSrc?: string, // neobdelana pot do slike
    logoSrc100px?: string, // spremenjeno za sličice
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
    signUpDate: number; // number zaradi "legacy" razlogov
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
    /** @readonly - Izračunano glede na packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Izračunano glede na packageId. **/
    hasAuditing?: boolean
    /** Z najemnikom lahko shranite par ključ-vrednost, ki ga lahko uporabite za poizvedovanje. Ključi ne smejo vsebovati "." ali "$", niti biti daljši od 100 znakov. Vrednosti ne smejo biti daljše od 2k znakov. **/
    meta?: Record<string, string | null>
}
[inline-code-end]