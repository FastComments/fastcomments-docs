The `TenantPackage` 定義可供 `Tenant` 使用的方案資訊。租戶可能有多個可用方案，但在任何時間點僅能有一個被使用。

A `Tenant` 在其 `packageId` 指向有效的 `TenantPackage` 前，無法用於任何產品。

有兩種類型的 `TenantPackage` 物件：

1. 固定定價方案 - 當 `hasFlexPricing` 為 false。
2. 彈性定價 - 當 `hasFlexPricing` 為 true。

在兩種情況下，帳戶使用該方案時都會定義限制，然而在彈性定價（Flex）下，租戶會被收取基本費用外加其實際使用量，使用量由 `flex*` 參數定義。

租戶可以擁有多個租戶方案，並可從 [帳單資訊頁面.](https://fastcomments.com/auth/my-account/billing-info) 自行變更方案。

如果您將自行為租戶處理帳務，您仍然需要為每個租戶定義一個方案以規定其限制。只要在 `Tenant` 上將 `billingHandledExternally` 設為 `true`，他們就無法自行變更帳務資訊或啟用中的方案。

您不得為子租戶建立高於父租戶限制的方案。

`TenantPackage` 物件的結構如下：

[inline-code-attrs-start title = 'TenantPackage 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // 每位一般 SSO 使用者的費用（不含管理者/版主權限）
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // 具有管理員權限的 SSO 使用者費用（isAccountOwner 或 isAdminAdmin 標記）
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // 具有版主權限的 SSO 使用者費用（isCommentModeratorAdmin 標記）
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]

---