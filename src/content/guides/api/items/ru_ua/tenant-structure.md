`Tenant` определяет клиента FastComments.com. Их можно создать через API арендаторами с доступом white-label. Арендаторы с white-label не могут создавать других арендаторов с white-label (разрешён только один уровень вложенности).

Структура объекта `Tenant` выглядит следующим образом:

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
    logoSrc100px?: string, // изменённый для миниатюр
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
    signUpDate: number; // число по "legacy" причинам
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
    /** Вы можете хранить пару ключ-значение у Tenant, которую можно использовать для запросов. Ключи не могут содержать "." или "$", или быть длиннее 100 символов. Значения не могут быть длиннее 2k символов. **/
    meta?: Record<string, string | null>
}
[inline-code-end]