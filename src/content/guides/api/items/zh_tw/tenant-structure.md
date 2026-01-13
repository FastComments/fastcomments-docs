The `Tenant` 定義 FastComments.com 的客戶。具有白標存取權的租戶可以透過 API 建立這些租戶。白標租戶無法建立其他白標租戶（僅允許一層巢狀）。

`Tenant` 物件的結構如下：

[inline-code-attrs-start title = '租戶結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** 這也可以透過 DomainConfig API 處理。 **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // 你可能想要使用 Unknown
    logoSrc?: string, // 原始圖片路徑
    logoSrc100px?: string, // 為縮圖調整大小
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
    signUpDate: number; // number 是因為「歷史」原因
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
    /** @readonly - 根據 packageId 計算。 **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - 根據 packageId 計算。 **/
    hasAuditing?: boolean
    /** 你可以在租戶上儲存鍵值對，並用於查詢。鍵不能包含 "." 或 "$"，或長於 100 個字元。值不得超過 2k 字元。 **/
    meta?: Record<string, string | null>
}
[inline-code-end]