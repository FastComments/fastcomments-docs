Das `TenantPackage` definiert Paketinformationen, die einem `Tenant` zur Verfügung stehen. Ein Tenant kann viele Pakete zur Verfügung haben, aber nur
eines zu einem bestimmten Zeitpunkt nutzen.

Ein `Tenant` kann für keine Produkte verwendet werden, bis seine `packageId` auf ein gültiges `TenantPackage` verweist.

Es gibt zwei Arten von `TenantPackage`-Objekten:

1. Pakete mit Festpreisen - wobei `hasFlexPricing` false ist.
2. Flexible Preisgestaltung - wobei `hasFlexPricing` true ist.

In beiden Fällen werden Limits für das Konto definiert, das das Paket verwendet, jedoch wird bei Flex dem Tenant ein Grundpreis plus
die Nutzung berechnet, definiert durch die `flex*`-Parameter.

Ein Tenant kann mehrere Tenant-Pakete haben und die Möglichkeit haben, das Paket selbst von der [Abrechnungsinfo-Seite](https://fastcomments.com/auth/my-account/billing-info) zu ändern.

Wenn Sie die Abrechnung für Tenants selbst übernehmen, müssen Sie dennoch ein Paket für jeden Tenant definieren, um deren Limits festzulegen. Setzen Sie einfach `billingHandledExternally` auf `true` beim `Tenant` und sie
werden nicht in der Lage sein, ihre Abrechnungsinformationen oder das aktive Paket selbst zu ändern.

Sie dürfen keine Pakete mit höheren Limits als der übergeordnete Tenant erstellen.

Die Struktur des `TenantPackage`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'TenantPackage Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
