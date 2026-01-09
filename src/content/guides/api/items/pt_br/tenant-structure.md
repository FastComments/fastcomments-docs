O `Tenant` define um cliente do FastComments.com. Eles podem ser criados via API por tenants com acesso a white labeling. Tenants white-labeled não podem criar outros tenants white-labeled (apenas um nível de aninhamento é permitido).

A estrutura para o objeto `Tenant` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Isso também pode ser tratado pela API DomainConfig. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // provavelmente você quer Unknown
    logoSrc?: string, // caminho bruto da imagem
    logoSrc100px?: string, // redimensionado para miniaturas
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
    signUpDate: number; // number por razões "legacy"
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
    /** @readonly - É calculado com base em packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - É calculado com base em packageId. **/
    hasAuditing?: boolean
    /** You can store a key value pair with the tenant which you can use to query. Keys cannot contain "." or "$", or be longer than 100 chars. Values may not be longer than 2k chars. **/
    meta?: Record<string, string | null>
}
[inline-code-end]