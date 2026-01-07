`TenantPackage` дефинира информация за пакет, достъпна за `Tenant`. Tenant може да има много налични пакети, но само
един в употреба в даден момент.

`Tenant` не може да се използва за никакви продукти, докато неговият `packageId` не сочи към валиден `TenantPackage`.

Има два типа обекти `TenantPackage`:

1. Пакети с фиксирана цена - където `hasFlexPricing` е false.
2. Гъвкаво ценообразуване - където `hasFlexPricing` е true.

И в двата случая се дефинират лимити на акаунта, използващ пакета, но при Flex tenant-ът се таксува базова цена плюс
това, което е използвал, дефинирано от `flex*` параметрите.

Tenant може да има множество tenant пакети и да има възможност сам да променя пакета от [страницата за информация за фактуриране.](https://fastcomments.com/auth/my-account/billing-info)

Ако ще се справяте сами с фактурирането за tenant-и, все пак ще трябва да дефинирате пакет за всеки tenant, за да дефинирате техните лимити. Просто задайте `billingHandledExternally` на `true` за `Tenant` и те
няма да могат сами да променят своята информация за фактуриране или активен пакет.

Не можете да създавате пакети с по-високи лимити от родителския tenant.

Структурата на обекта `TenantPackage` е следната:

[inline-code-attrs-start title = 'Структура на TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
