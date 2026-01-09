The `Tenant` 定义了一个 FastComments.com 客户。具有白标访问权限的租户可以通过 API 创建它们。白标租户
不能创建其他白标租户（只允许一层嵌套）。

The structure for the `Tenant` object is as follows:

[inline-code-attrs-start title = '租户结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** 这也可以通过 DomainConfig API 处理。 **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // 你可能想要 Unknown
    logoSrc?: string, // 原始图像路径
    logoSrc100px?: string, // 为缩略图调整大小
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
    signUpDate: number; // 由于“遗留”原因为 number
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
    /** @readonly - 基于 packageId 计算。 **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - 基于 packageId 计算。 **/
    hasAuditing?: boolean
    /** 你可以在租户上存储一个键值对并用于查询。键不能包含 "." 或 "$"，或超过 100 个字符。值不能超过 2k 个字符。 **/
    meta?: Record<string, string | null>
}
[inline-code-end]