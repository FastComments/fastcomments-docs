`Tenant` дефинише корисника FastComments.com. Они се могу креирати преко API-ја од стране tenant-ова који имају приступ за white labeling. White labeled tenants
cannot create other white labeled tenants (only one level of nesting is allowed).

Структура објекта `Tenant` је следећа:

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
    siteType?: FastCommentsSiteType, // вероватно желите Unknown
    logoSrc?: string, // путања до оригиналне слике
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
    /** Можете да сачувате пару кључ/вредност за tenant који можете користити за упите. Кључеви не смеју да садрже "." или "$", нити да буду дужи од 100 карактера. Вредности не смеју бити дужи од 2000 карактера. **/
    meta?: Record<string, string | null>
}
[inline-code-end]

---