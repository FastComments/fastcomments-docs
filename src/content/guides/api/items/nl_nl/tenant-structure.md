De `Tenant` definieert een FastComments.com-klant. Ze kunnen via de API worden aangemaakt door tenants met white-labeling toegang. White-labeled tenants kunnen geen andere white-labeled tenants aanmaken (er is slechts één niveau van nesting toegestaan).

De structuur voor het `Tenant`-object is als volgt:

[inline-code-attrs-start title = 'Tenant-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** Dit kan ook via de DomainConfig API worden afgehandeld. **/
export interface TenantDomainConfig {
    domain: string
    emailFromName?: string
    emailFromEmail?: string
    createdAt?: string,
    siteType?: FastCommentsSiteType, // je wilt waarschijnlijk Unknown
    logoSrc?: string, // pad naar ruwe afbeelding
    logoSrc100px?: string, // verkleind voor miniaturen
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
    signUpDate: number; // number vanwege "legacy" redenen
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
    /** @readonly - Wordt berekend op basis van packageId. **/
    hasFlexPricing?: boolean
    /** @readonly **/
    flexLastBilledAmount?: number
    /** @readonly - Wordt berekend op basis van packageId. **/
    hasAuditing?: boolean
    /** Je kunt een sleutel-waarde-paar opslaan bij de tenant dat je kunt gebruiken om te zoeken. Sleutels mogen geen "." of "$" bevatten, of langer zijn dan 100 tekens. Waarden mogen niet langer zijn dan 2k tekens. **/
    meta?: Record<string, string | null>
}
[inline-code-end]

---