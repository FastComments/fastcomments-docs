The `TenantPackage` дефинише информације о пакету доступном за `Tenant`. Тенант може имати више пакета доступних, али се у датом тренутку користи само један.

A `Tenant` не може да се користи за било који производ док његов `packageId` не упућује на важећи `TenantPackage`.

Постоје две врсте објеката `TenantPackage`:

1. Пакети са фиксним ценама - где је `hasFlexPricing` false.
2. Пакети са флексибилним ценама - где је `hasFlexPricing` true.

У оба случаја, лимити су дефинисани за налог који користи пакет, међутим код флекс пакета тенанту се наплаћује основна цена плус коришћено, дефинисано `flex*` параметрима.

Тенант може имати више пакета и има могућност да сам промијени пакет преко [Странице са информацијама о наплати.](https://fastcomments.com/auth/my-account/billing-info)

Ако ћете сами обрађивати наплату за тенанте, и даље морате дефинисати пакет за сваког тенанта како бисте дефинисали њихове лимите. Једноставно подесите `billingHandledExternally` на `true` на `Tenant` и они неће моћи сами да мењају своје податке за наплату, или активни пакет.

Не смете креирати пакете са већим лимитима од матичног тенанта.

Структура објекта `TenantPackage` је следећа:

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
    flexSSOUserCostCents?: null // Цена по редовном SSO кориснику (без админ/модераторских дозвола)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Цена по SSO кориснику са администраторским дозволама (ознаке isAccountOwner или isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Цена по SSO кориснику са модераторским дозволама (ознака isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]