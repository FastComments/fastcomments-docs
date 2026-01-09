The `Tenant` дефинише FastComments.com клијента. Они се могу креирати преко API-ја од стране tenants који имају приступ за white labeling.
White labeled tenants
cannot create other white labeled tenants (only one level of nesting is allowed).

The structure for the `Tenant` object is as follows:

[inline-code-attrs-start title = 'Структура Tenant-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Ово се такође може обрадити преко DomainConfig API-ја. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // вероватно ћете желети Unknown
    logoSrc?: string, // оригинални пут до слике
    logoSrc100px?: string, // промењено за минијатуре
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
    /** You can store a key value pair with the tenant which you can use to query. Keys cannot contain "." or "$", or be longer than 100 chars. Values may not be longer than 2k chars. **/
    meta?: Record<string, string | null>
}
[inline-code-end]