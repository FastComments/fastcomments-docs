`Tenant` определяет клиента FastComments.com. Такие объекты могут быть созданы через API тенантами с доступом к white labeling. White labeled тенанты
не могут создавать других white labeled тенантов (допускается только один уровень вложенности).

Структура объекта `Tenant` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Это также можно настроить через API DomainConfig. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // вам, скорее всего, нужен Unknown
    logoSrc?: string, // исходный путь к изображению
    logoSrc100px?: string, // уменьшенная версия для миниатюр
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
    signUpDate: number; // число из-за "устаревших" причин
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
    /** Вы можете сохранить пару ключ-значение вместе с тенантом и использовать её для запросов. Ключи не могут содержать "." или "$", а также быть длиннее 100 символов. Значения не могут быть длиннее 2 тыс. символов. **/
    meta?: Record<string, string | null>
}
[inline-code-end]