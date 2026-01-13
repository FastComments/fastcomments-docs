---
`TenantPackage` 定义了可供 `Tenant` 使用的套餐信息。一个租户可能有多个可用的套餐，但在任意时刻只有一个处于使用中。

`Tenant` 在其 `packageId` 指向一个有效的 `TenantPackage` 之前，不能用于任何产品。

有两种类型的 `TenantPackage` 对象：

1. 固定定价包 - 当 `hasFlexPricing` 为 false 时。
2. 灵活定价 - 当 `hasFlexPricing` 为 true 时。

在两种情况下，使用该套餐的账号都会定义限制，然而在灵活（Flex）定价中，租户会被收取基础费用加上其实际使用量，使用 `flex*` 参数来定义。

租户可以拥有多个租户套餐，并可以从 [账单信息页面.](https://fastcomments.com/auth/my-account/billing-info) 自行更改所使用的套餐。

如果您将自行为租户处理计费，您仍需为每个租户定义一个套餐以规定其限额。只需在 `Tenant` 上将 `billingHandledExternally` 设置为 `true`，他们就无法自行更改其账单信息或当前使用的套餐。

您不得创建比父租户限额更高的套餐。

`TenantPackage` 对象的结构如下：

[inline-code-attrs-start title = 'TenantPackage 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // 每个普通 SSO 用户的费用（没有管理员/版主权限）
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // 拥有管理员权限的 SSO 用户的费用（isAccountOwner 或 isAdminAdmin 标志）
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // 拥有版主权限的 SSO 用户的费用（isCommentModeratorAdmin 标志）
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]

---