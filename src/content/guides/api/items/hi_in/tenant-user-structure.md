`TenantUser` एक `User` को परिभाषित करता है जो एक विशिष्ट टेनेंट द्वारा प्रबंधित होता है। उनका खाता उस टेनेंट पर पूर्ण नियंत्रण रखता है जिससे वे संबद्ध हैं, और उनके खाते को [UI](https://fastcomments.com/auth/my-account/users) या API के माध्यम से अपडेट या डिलीट किया जा सकता है।

टेनेंट उपयोगकर्ता `Tenant` तक सभी अनुमतियों और पहुँच वाले व्यवस्थापक हो सकते हैं, या उन्हें टिप्पणियों के संचालन, API कुंजियों तक पहुँच, आदि जैसी विशिष्ट अनुमतियों तक सीमित किया जा सकता है।

`TenantUser` ऑब्जेक्ट की संरचना इस प्रकार है:

[inline-code-attrs-start title = 'TenantUser संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** यह सूचनाओं (notifications) के लिए है। **/
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
    /** उदाहरण के लिए, टिप्पणीकार के ब्लॉग का एक लिंक। **/
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
    /** क्या उपयोगकर्ता टिप्पणीकारों से सहायता अनुरोध प्राप्त करता है? **/
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