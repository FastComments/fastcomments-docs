El `Tenant` define un cliente de FastComments.com. Pueden ser creados a través de la API por inquilinos con acceso de marca blanca. Los inquilinos de marca blanca
no pueden crear otros inquilinos de marca blanca (solo se permite un nivel de anidación).

La estructura del objeto `Tenant` es la siguiente:

[inline-code-attrs-start title = 'Estructura de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** This can also be handled via the DomainConfig API. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // you probably want Unknown
    logoSrc?: string, // raw image path
    logoSrc100px?: string, // resized for thumbnails
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
    signUpDate: number; // number due to "legacy" reasons
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
    /** @readonly - Is calculated based on packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Is calculated based on packageId. **/
    hasAuditing?: boolean
    /** You can store a key value pair with the tenant which you can use to query. Keys cannot contain "." or "$", or be longer than 100 chars. Values may not be longer than 2k chars. **/
    meta?: Record<string, string | null>
}
[inline-code-end]
