The `Tenant` definira korisnika FastComments.com. Mogu ih stvarati putem API-ja tenanti s pristupom za white labeling. Tenanti s bijelim označavanjem
ne mogu stvarati druge tenante s bijelim označavanjem (dopuštena je samo jedna razina ugnježđivanja).

The structure for the `Tenant` object is as follows:

[inline-code-attrs-start title = 'Struktura Tenanta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Ovo se također može obraditi putem DomainConfig API-ja. **/
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
    signUpDate: number; // broj zbog razloga vezanih uz 'legacy'
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
    /** @readonly - Izračunava se na temelju packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Izračunava se na temelju packageId. **/
    hasAuditing?: boolean
    /** Možete pohraniti par ključ-vrijednost s tenantom koji možete koristiti za upite. Ključevi ne smiju sadržavati "." ili "$", ili biti dulji od 100 znakova. Vrijednosti ne smiju biti dulje od 2k znakova. **/
    meta?: Record<string, string | null>
}
[inline-code-end]

---