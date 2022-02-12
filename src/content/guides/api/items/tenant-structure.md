The `Tenant` defines a FastComments.com customer. They can be created via the API by tenants with white labeling access. White labeled tenants
cannot create other white labeled tenants (only one level of nesting is allowed).

The structure for the `Tenant` object is as follows:

[inline-code-attrs-start title = 'Tenant Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export enum SiteType {
    Unknown = 0,
    WordPress = 1
}

/** This can also be handled via the DomainConfig API. **/
export interface TenantDomainConfig {
    domain: string;
    emailFromName?: string;
    emailFromEmail?: string;
    createdAt?: string,
    siteType?: FastCommentsSiteType, // you probably want Unknown
    logoSrc?: string, // raw image path
    logoSrc100px?: string, // resized for thumbnails
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
    name: string;
    address: string;
    city: string;
    state: string;
    zip: string;
    country: string;
}

export enum TenantPaymentFrequency {
    Monthly = 0,
    Annually = 1
}

export interface Tenant {
    id: string;
    name: string;
    email: string;
    signUpDate: number; // number due to "legacy" reasons
    packageId?: string | null;
    paymentFrequency?: TenantPaymentFrequency;
    billingInfoValid?: boolean;
    billingHandledExternally?: boolean;
    createdBy?: string;
    isSetup?: boolean;
    domainConfiguration: FastCommentsAPITenantDomainConfig[];
    billingInfo?: FastCommentsAPITenantBillingInfo;
    stripeCustomerId?: string;
    stripeSubscriptionId?: string;
    stripePlanId?: string;
    enableProfanityFilter?: boolean;
    enableSpamFilter?: boolean;
    lastBillingIssueReminderDate?: string;
    removeUnverifiedComments?: boolean;
    unverifiedCommentsTTLms?: number;
    commentsRequireApproval?: boolean;
    autoApproveCommentOnVerification?: boolean;
    sendProfaneToSpam?: boolean;
    /** @readonly - Is calculated based on packageId. **/
    hasFlexPricing?: boolean;
    /** @readonly **/
    flexLastBilledAmount?: number;
}
[inline-code-end]
