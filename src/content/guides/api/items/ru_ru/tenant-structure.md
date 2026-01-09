The `Tenant` определяет клиента FastComments.com. Их можно создавать через API арендаторами с доступом white-label. Тенанты с white-label
не могут создавать других тенантов с white-label (разрешён только один уровень вложенности).

The structure for the `Tenant` object is as follows:

[inline-code-attrs-start title = 'Структура Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Это также можно обработать через DomainConfig API. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // вероятно, вы захотите Unknown
    logoSrc?: string, // путь к исходному изображению
    logoSrc100px?: string, // масштабировано для миниатюр
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
    signUpDate: number; // число по причинам совместимости с «legacy»
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
    /** @readonly - Вычисляется на основе packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Вычисляется на основе packageId. **/
    hasAuditing?: boolean
    /** You can store a key value pair with the tenant which you can use to query. Keys cannot contain "." or "$", or be longer than 100 chars. Values may not be longer than 2k chars. **/
    meta?: Record<string, string | null>
}
[inline-code-end]