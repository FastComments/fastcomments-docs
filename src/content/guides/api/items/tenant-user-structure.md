The `TenantUser` defines a `User` which is managed by a specific tenant. Their account is in complete control of the tenant
they are associated with, and their account can be updated or deleted via the [UI](https://fastcomments.com/auth/my-account/users) or API.

Tenant users can be administrators with all permissions and access to the `Tenant`, or they can be limited to specific permissions to
moderate comments, access API keys, etc.

The structure for the `TenantUser` object is as follows:

[inline-code-attrs-start title = 'TenantUser Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** This is for notifications. **/
export enum UserDigestEmailFrequency {
    Disabled = -1,
    Daily = 0,
    Weekly = 1,
    Monthly = 2
}

export interface TenantUser {
    id: string;
    tenantId: string;
    username: string;
    /** A link to the commenter's blog, for example. **/
    websiteUrl?: string;
    email: string;
    signUpDate: number;
    createdFromUrlId: string;
    createdFromTenantId: string;
    verified: boolean;
    loginCount: number;
    optedInNotifications: boolean;
    optedInTenantNotifications: boolean;
    hideAccountCode: boolean;
    avatarSrc?: string;
    /** Does the user receive help requests from commenters? **/
    isHelpRequestAdmin: boolean;
    isAccountOwner: boolean;
    isAdminAdmin: boolean;
    isBillingAdmin: boolean;
    isAnalyticsAdmin: boolean;
    isCustomizationAdmin: boolean;
    isManageDataAdmin: boolean;
    isCommentModeratorAdmin: boolean;
    isAPIAdmin: boolean;
    moderatorIds: string[];
    locale: FastCommentsLocale;
    digestEmailFrequency: UserDigestEmailFrequency;
    lastLoginDate: string;
    displayLabel?: string;
}
[inline-code-end]
