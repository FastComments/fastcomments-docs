The `TenantPackage` defines package information available to a `Tenant`. A tenant may have many packages available, but only
one in use at a given time.

A `Tenant` cannot be used for any products until its `packageId` points to a valid `TenantPackage`.

There are two types of `TenantPackage` objects:

1. Fixed-pricing packages - where `hasFlexPricing` is false.
2. Flexible pricing - where `hasFlexPricing` is true.

In both case limits are defined on the account using the package, however with Flex the tenant is charged a base price plus
what they used, defined by the `flex*` parameters.

A tenant may have multiple tenant packages and have the ability to change the package themselves from the [Billing Info Page.](https://fastcomments.com/auth/my-account/billing-info)

If you will be handling billing for tenants yourselves, you will still need to define a package for each tenant to define their limits. Simply set `billingHandledExternally` to `true` on the `Tenant` and they
will not be able to change their billing information, or active package, themselves.

You may not create packages with higher limits than the parent tenant.

The structure for the `TenantPackage` object is as follows:

[inline-code-attrs-start title = 'TenantPackage Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
