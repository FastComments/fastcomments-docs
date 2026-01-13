`TenantPackage` מגדיר מידע על חבילה זמינה ל-`Tenant`. לשוכר עשויות להיות חבילות רבות זמינות, אבל רק
אחת בשימוש בזמן נתון.

לא ניתן להשתמש ב-`Tenant` עבור מוצרים כלשהם עד ש-`packageId` שלו מצביע על `TenantPackage` חוקי.

ישנם שני סוגים של אובייקטי `TenantPackage`:

1. חבילות מחיר קבוע - שבהן `hasFlexPricing` הוא false.
2. תמחור גמיש - שבו `hasFlexPricing` הוא true.

בשני המקרים מוגדרות מגבלות על החשבון שמשתמש בחבילה, אולם עם Flex השוכר מחויב במחיר בסיס בתוספת
מה שהשתמש, המוגדר על ידי פרמטרי `flex*`.

לשוכר עשויות להיות מספר חבילות שוכר ויכולת לשנות את החבילה בעצמו מ-[דף פרטי החיוב.](https://fastcomments.com/auth/my-account/billing-info)

אם אתה מתכנן לטפל בחיוב עבור שוכרים בעצמך, עדיין תצטרך להגדיר חבילה לכל שוכר כדי להגדיר את המגבלות שלהם. פשוט הגדר `billingHandledExternally` ל-`true` על ה-`Tenant` והם
לא יוכלו לשנות את פרטי החיוב שלהם, או את החבילה הפעילה, בעצמם.

אתה לא יכול ליצור חבילות עם מגבלות גבוהות יותר מהשוכר האב.

המבנה עבור אובייקט `TenantPackage` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface TenantPackage {
    id: string
    name: string
    tenantId: string
    createdAt: string
    monthlyCostUSD: number
    yearlyCostUSD: number
    monthlyStripePlanId?: string
    yearlyStripePlanId?: string
    maxMonthlyPageLoads: number
    maxMonthlyAPICredits: number
    maxMonthlyComments: number
    maxConcurrentUsers: number
    maxTenantUsers: number
    maxSSOUsers: number
    maxModerators: number
    maxDomains: number
    maxWhiteLabeledTenants: number
    hasWhiteLabeling: boolean
    hasDebranding: boolean
    forWhoText: string
    featureTaglines: string[]
    hasAuditing: boolean
    hasFlexPricing: boolean
    flexPageLoadCostCents?: null
    flexPageLoadUnit?: null
    flexCommentCostCents?: null
    flexCommentUnit?: null
    flexSSOUserCostCents?: null // Cost per regular SSO user (without admin/moderator permissions)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Cost per SSO user with admin permissions (isAccountOwner or isAdminAdmin flags)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Cost per SSO user with moderator permissions (isCommentModeratorAdmin flag)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]
