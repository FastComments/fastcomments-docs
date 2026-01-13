`TenantUser` מגדיר `User` המנוהל על ידי שוכר ספציפי. החשבון שלו בשליטה מלאה של השוכר
שהוא משויך אליו, והחשבון שלו ניתן לעדכון או מחיקה דרך [ממשק המשתמש](https://fastcomments.com/auth/my-account/users) או ה-API.

משתמשי שוכר יכולים להיות מנהלים עם כל ההרשאות וגישה ל-`Tenant`, או שהם יכולים להיות מוגבלים להרשאות ספציפיות ל
ניהול תגובות, גישה למפתחות API, וכו'.

המבנה עבור אובייקט `TenantUser` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
