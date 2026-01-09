`Tenant` визначає клієнта FastComments.com. Вони можуть бути створені через API орендарями з доступом до white labeling. White labeled tenants
не можуть створювати інших white labeled tenants (дозволено тільки один рівень вкладеності).

Структура об'єкта `Tenant` виглядає так:

[inline-code-attrs-start title = 'Структура Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Це також можна обробити через DomainConfig API. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // ймовірно, ви хочете Unknown
    logoSrc?: string, // шлях до необробленого зображення
    logoSrc100px?: string, // змінений розмір для ескізів
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
    signUpDate: number; // number через "legacy" причини
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
    /** @readonly - Обчислюється на основі packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Обчислюється на основі packageId. **/
    hasAuditing?: boolean
    /** Ви можете зберегти у tenant пару ключ-значення, яку можна використовувати для запитів. Ключі не можуть містити "." або "$", або бути довшими за 100 символів. Значення не можуть бути довшими за 2000 символів. **/
    meta?: Record<string, string | null>
}
[inline-code-end]