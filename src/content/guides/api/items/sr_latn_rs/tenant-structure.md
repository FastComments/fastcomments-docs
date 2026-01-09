`Tenant` definiše FastComments.com kupca. Mogu se kreirati preko API-ja od strane tenanta koji imaju pristup za white labeling. White labeled tenanti ne mogu kreirati druge white labeled tenante (dozvoljen je samo jedan nivo ugnježđivanja).

Struktura `Tenant` objekta je sledeća:

[inline-code-attrs-start title = 'Struktura Tenant objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Ovo se takođe može rešiti putem DomainConfig API-ja. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // verovatno želite Unknown
    logoSrc?: string, // putanja do originalne slike
    logoSrc100px?: string, // promenjena veličina za sličice
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
    /** @readonly - Izračunava se na osnovu packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Izračunava se na osnovu packageId. **/
    hasAuditing?: boolean
    /** Možete sačuvati par ključ-vrednost sa tenant-om koji možete koristiti za upite. Ključevi ne mogu sadržati "." ili "$", ili biti duži od 100 karaktera. Vrednosti ne mogu biti duže od 2k karaktera. **/
    meta?: Record<string, string | null>
}
[inline-code-end]