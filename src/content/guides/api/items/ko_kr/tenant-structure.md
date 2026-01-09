`Tenant`는 FastComments.com의 고객을 정의합니다. 화이트 라벨링 권한이 있는 테넌트는 API를 통해 생성할 수 있습니다. 화이트 라벨 테넌트는 다른 화이트 라벨 테넌트를 생성할 수 없습니다(중첩은 한 단계만 허용됩니다).

다음은 `Tenant` 객체의 구조입니다:

[inline-code-attrs-start title = '테넌트 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** 이는 DomainConfig API로도 처리할 수 있습니다. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // 아마 Unknown을 원할 것입니다
    logoSrc?: string, // 원본 이미지 경로
    logoSrc100px?: string, // 썸네일용으로 리사이즈됨
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
    signUpDate: number; // '레거시' 이유로 number 타입입니다
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
    /** @readonly - packageId를 기반으로 계산됩니다. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - packageId를 기반으로 계산됩니다. **/
    hasAuditing?: boolean
    /** 테넌트에 키-값 쌍을 저장하여 쿼리에 사용할 수 있습니다. 키에는 "." 또는 "$"를 포함할 수 없고 길이는 100자를 초과할 수 없습니다. 값은 2k 문자를 초과할 수 없습니다. **/
    meta?: Record<string, string | null>
}
[inline-code-end]