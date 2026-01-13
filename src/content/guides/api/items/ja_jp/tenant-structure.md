`Tenant` は FastComments.com の顧客を定義します。ホワイトラベリング権限を持つテナントは API を介してそれらを作成できます。ホワイトラベル化テナントは他のホワイトラベル化テナントを作成できません（ネストは1レベルのみ許可されます）。

`Tenant` オブジェクトの構造は次の通りです:

[inline-code-attrs-start title = 'テナント構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** これは DomainConfig API でも処理できます。 **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // おそらく Unknown を使用するでしょう
    logoSrc?: string, // 元の画像パス
    logoSrc100px?: string, // サムネイル用にリサイズ済み
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
    signUpDate: number; // number 型は "legacy" の理由による
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
    /** @readonly - packageId に基づいて計算されます。 **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - packageId に基づいて計算されます。 **/
    hasAuditing?: boolean
    /** テナントにキーと値のペアを保存でき、これをクエリに使用できます。キーには "." または "$" を含めることはできず、100 文字を超えることはできません。値は 2k 文字を超えることはできません。 **/
    meta?: Record<string, string | null>
}
[inline-code-end]

---