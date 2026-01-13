Ο `TenantUser` ορίζει έναν `User` που διαχειρίζεται από έναν συγκεκριμένο ενοικιαστή. Ο λογαριασμός του είναι υπό πλήρη έλεγχο του ενοικιαστή
με τον οποίο συσχετίζεται, και ο λογαριασμός του μπορεί να ενημερωθεί ή να διαγραφεί μέσω του [UI](https://fastcomments.com/auth/my-account/users) ή του API.

Οι χρήστες ενοικιαστή μπορούν να είναι διαχειριστές με όλα τα δικαιώματα και πρόσβαση στον `Tenant`, ή μπορούν να περιοριστούν σε συγκεκριμένα δικαιώματα για
τον έλεγχο σχολίων, πρόσβαση σε κλειδιά API, κλπ.

Η δομή για το αντικείμενο `TenantUser` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** This is for notifications. **/
export enum UserDigestEmailFrequency {
    Disabled = -1,
    Daily = 0,
    Weekly = 1,
    Monthly = 2
}

export interface TenantUser {
    id: string
    tenantId: string
    username: string
    /** A link to the commenter's blog, for example. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    verified: boolean
    loginCount: number
    optedInNotifications: boolean
    optedInTenantNotifications: boolean
    hideAccountCode: boolean
    avatarSrc?: string
    /** Does the user receive help requests from commenters? **/
    isHelpRequestAdmin: boolean
    isAccountOwner: boolean
    isAdminAdmin: boolean
    isBillingAdmin: boolean
    isAnalyticsAdmin: boolean
    isCustomizationAdmin: boolean
    isManageDataAdmin: boolean
    isCommentModeratorAdmin: boolean
    isAPIAdmin: boolean
    moderatorIds: string[]
    locale: FastCommentsLocale
    digestEmailFrequency: UserDigestEmailFrequency
    lastLoginDate: string
    displayLabel?: string
    karma?: number
}
[inline-code-end]
