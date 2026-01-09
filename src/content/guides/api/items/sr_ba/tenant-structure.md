The `Tenant` дефинише FastComments.com купца. Они се могу креирати преко API-ја од стране тенаната који имају white-label приступ. Тенанти са white-label приступом не могу креирати друге тенанте са white-label (дозвољен је само један ниво угнежђивања).

Структура за објекат `Tenant` је следећа:

[inline-code-attrs-start title = 'Структура Tenant-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Ово се такође може обрадити преко DomainConfig API. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // највероватније ћете желети Unknown
    logoSrc?: string, // необрађена путања до слике
    logoSrc100px?: string, // прилагођено за минијатуре
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
    signUpDate: number; // број због "legacy" разлога
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
    /** @readonly - Израчунава се на основу packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Израчунава се на основу packageId. **/
    hasAuditing?: boolean
    /** Можете похранити пар кључ-вриједност уз тенанта који можете користити за упит. Кључеви не могу садржати "." или "$", нити бити дужи од 100 знакова. Вриједности не могу бити дужи од 2k знакова. **/
    meta?: Record<string, string | null>
}
[inline-code-end]

---