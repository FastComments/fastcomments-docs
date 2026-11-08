`Tenant` एक FastComments.com ग्राहक (customer) को परिभाषित करता है। इन्हें API के माध्यम से उन tenants द्वारा बनाया जा सकता है जिनके पास white labeling एक्सेस है। White labeled tenants अन्य white labeled tenants नहीं बना सकते (केवल एक स्तर की nesting की अनुमति है)।

`Tenant` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'Tenant संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** इसे DomainConfig API के माध्यम से भी संभाला जा सकता है। **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // आप शायद Unknown चाहेंगे
    logoSrc?: string, // रॉ इमेज पाथ
    logoSrc100px?: string, // थंबनेल के लिए रिसाइज़ किया गया
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
    signUpDate: number; // "legacy" कारणों से number
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
    /** @readonly - packageId के आधार पर गणना की जाती है। **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - packageId के आधार पर गणना की जाती है। **/
    hasAuditing?: boolean
    /** आप tenant के साथ एक key value pair स्टोर कर सकते हैं जिसका उपयोग आप क्वेरी के लिए कर सकते हैं। Keys में "." या "$" नहीं हो सकता, और 100 वर्णों से लंबा नहीं हो सकता। Values 2k वर्णों से लंबी नहीं हो सकतीं। **/
    meta?: Record<string, string | null>
}
[inline-code-end]