`Tenant` definiuje klienta FastComments.com. Mogą być tworzone za pomocą API przez tenantów z dostępem do white labelingu. Tenanci z white-labelingiem
nie mogą tworzyć innych tenantów z white-labelingiem (dozwolony jest tylko jeden poziom zagnieżdżenia).

Struktura obiektu `Tenant` jest następująca:

[inline-code-attrs-start title = 'Struktura obiektu Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** To może być również obsługiwane przez API DomainConfig. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // prawdopodobnie chcesz Unknown
    logoSrc?: string, // surowa ścieżka obrazu
    logoSrc100px?: string, // przeskalowane dla miniatur
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
    signUpDate: number; // number ze względów „legacy”
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
    /** @readonly - Obliczane na podstawie packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Obliczane na podstawie packageId. **/
    hasAuditing?: boolean
    /** Możesz przechowywać parę klucz-wartość z tenantem, której możesz użyć do zapytań. Klucze nie mogą zawierać "." ani "$", lub być dłuższe niż 100 znaków. Wartości nie mogą być dłuższe niż 2k znaków. **/
    meta?: Record<string, string | null>
}
[inline-code-end]

---