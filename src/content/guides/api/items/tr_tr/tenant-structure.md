---
`Tenant`, FastComments.com müşterisini tanımlar. Bunlar API aracılığıyla white-label erişimine sahip kiracılar tarafından oluşturulabilir. White-label kiracılar
başka white-label kiracılar oluşturamaz (sadece bir düzey iç içe izin verilir).

`Tenant` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'Tenant Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Bu, DomainConfig API aracılığıyla da işlenebilir. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // muhtemelen Unknown kullanmak istersiniz
    logoSrc?: string, // ham resim yolu
    logoSrc100px?: string, // küçük resimler için yeniden boyutlandırılmış
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
    signUpDate: number; // sayı, "legacy" nedenlerden dolayı
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
    /** @readonly - packageId'ye göre hesaplanır. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - packageId'ye göre hesaplanır. **/
    hasAuditing?: boolean
    /** Kiracıyla sorgulamada kullanabileceğiniz bir anahtar-değer çifti saklayabilirsiniz. Anahtarlar "." veya "$" içeremez veya 100 karakterden uzun olamaz. Değerler 2k karakterden uzun olamaz. **/
    meta?: Record<string, string | null>
}
[inline-code-end]

---