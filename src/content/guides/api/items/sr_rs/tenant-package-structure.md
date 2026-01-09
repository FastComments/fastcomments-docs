The `TenantPackage` дефинише информације о пакету доступном `Tenant`-у. Тенант може имати више доступних пакета, али само један може бити активан у датом тренутку.

`Tenant` не може да се користи за било које производе док његов `packageId` не показује на важећи `TenantPackage`.

Постоје два типа `TenantPackage` објеката:

1. Пакети са фиксном ценом - где је `hasFlexPricing` false.
2. Флексибилно ценообразовање - где је `hasFlexPricing` true.

У оба случаја лимити се дефинишу на налогу који користи пакет, међутим са Flex моделом тенант се наплаћује основна цена плус оно што је користио, дефинисано `flex*` параметрима.

Тенант може имати више tenant пакета и има могућност да сам промени пакет са странице [Billing Info Page.](https://fastcomments.com/auth/my-account/billing-info)

Ако ћете сами управљати наплатом за тенанте, и даље ћете морати да дефинишете пакет за сваког тенанта како бисте одредили њихове лимите. Једноставно подесите `billingHandledExternally` на `true` на `Tenant`-у и они неће моћи сами да мењају своје податке о наплати или активни пакет.

Не можете креирати пакете са вишим лимитима него родитељски тенант.

Структура `TenantPackage` објекта је следећа:

[inline-code-attrs-start title = 'Структура TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // Цена по редовном SSO кориснику (без админ/модератор права)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Цена по SSO кориснику са админским правима (флагови isAccountOwner или isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Цена по SSO кориснику са модераторским правима (флаг isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]