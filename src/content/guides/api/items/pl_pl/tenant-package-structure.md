The `TenantPackage` definiuje informacje o pakiecie dostępne dla `Tenant`. Tenant może mieć wiele dostępnych pakietów, ale tylko
jeden używany w danym momencie.

`Tenant` nie może być używany do żadnych produktów, dopóki jego `packageId` nie wskazuje na prawidłowy `TenantPackage`.

Istnieją dwa typy obiektów `TenantPackage`:

1. Pakiety o stałej cenie - gdzie `hasFlexPricing` jest false.
2. Elastyczne wycenianie - gdzie `hasFlexPricing` jest true.

W obu przypadkach limity są definiowane na koncie używającym pakietu, jednak w przypadku Flex tenant jest obciążany ceną podstawową plus
tym, co zużył, określonym przez parametry `flex*`.

Tenant może mieć wiele pakietów tenant i mieć możliwość zmiany pakietu samodzielnie ze [Strony informacji o rozliczeniach.](https://fastcomments.com/auth/my-account/billing-info)

Jeśli będziecie sami obsługiwać rozliczenia za tenantów, nadal będziecie musieli zdefiniować pakiet dla każdego tenant, aby określić ich limity. Po prostu ustaw `billingHandledExternally` na `true` w `Tenant` i
nie będą mogli samodzielnie zmieniać swoich informacji rozliczeniowych ani aktywnego pakietu.

Nie można tworzyć pakietów z wyższymi limitami niż tenant nadrzędny.

Struktura obiektu `TenantPackage` jest następująca:

[inline-code-attrs-start title = 'Struktura TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // Koszt za zwykłego użytkownika SSO (bez uprawnień administratora/moderatora)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Koszt za użytkownika SSO z uprawnieniami administratora (flagi isAccountOwner lub isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Koszt za użytkownika SSO z uprawnieniami moderatora (flaga isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]