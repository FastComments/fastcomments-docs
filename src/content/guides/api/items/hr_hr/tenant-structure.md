`Tenant` definira FastComments.com korisnika. Mogu se stvoriti putem API-ja od strane tenanata s pristupom white labelingu. White labelirani tenanti
ne mogu stvarati druge white labelirane tenante (dopuštena je samo jedna razina ugniježđivanja).

Struktura objekta `Tenant` je sljedeća:

[inline-code-attrs-start title = 'Struktura Tenant-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Ovo se također može riješiti putem DomainConfig API-ja. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // vjerojatno želite Unknown
    logoSrc?: string, // sirova putanja slike
    logoSrc100px?: string, // promijenjena veličina za sličice
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
    signUpDate: number; // broj zbog "legacy" razloga
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
    /** @readonly - Računa se na temelju packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Računa se na temelju packageId. **/
    hasAuditing?: boolean
    /** Možete pohraniti par ključ-vrijednost s tenantom koji možete koristiti za upite. Ključevi ne smiju sadržavati "." ili "$", niti biti dulji od 100 znakova. Vrijednosti ne smiju biti dulje od 2k znakova. **/
    meta?: Record<string, string | null>
}
[inline-code-end]