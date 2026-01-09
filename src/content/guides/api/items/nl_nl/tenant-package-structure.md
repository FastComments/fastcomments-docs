De `TenantPackage` definieert pakketinformatie die beschikbaar is voor een `Tenant`. Een tenant kan meerdere pakketten beschikbaar hebben, maar slechts
één tegelijk in gebruik.

Een `Tenant` kan niet voor producten worden gebruikt totdat zijn `packageId` naar een geldige `TenantPackage` verwijst.

Er zijn twee soorten `TenantPackage`-objecten:

1. Pakketten met vaste prijs - waarbij `hasFlexPricing` false is.
2. Flexibele prijsstelling - waarbij `hasFlexPricing` true is.

In beide gevallen worden limieten gedefinieerd op het account dat het pakket gebruikt, maar bij Flex wordt de tenant een basisprijs in rekening gebracht plus
wat ze gebruikt hebben, gedefinieerd door de `flex*` parameters.

Een tenant kan meerdere tenant-pakketten hebben en kan het pakket zelf wijzigen vanaf de [Pagina met facturatiegegevens.](https://fastcomments.com/auth/my-account/billing-info)

Als u zelf de facturering voor tenants afhandelt, moet u nog steeds voor elke tenant een pakket definiëren om hun limieten vast te leggen. Stel eenvoudig `billingHandledExternally` in op `true` op de `Tenant` en zij
zullen hun facturatiegegevens of actieve pakket niet zelf kunnen wijzigen.

U mag geen pakketten aanmaken met hogere limieten dan de bovenliggende tenant.

De structuur van het `TenantPackage`-object is als volgt:

[inline-code-attrs-start title = 'Structuur van TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // Kosten per reguliere SSO-gebruiker (zonder admin/moderatorrechten)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Kosten per SSO-gebruiker met admin-rechten (isAccountOwner or isAdminAdmin flags)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Kosten per SSO-gebruiker met moderatorrechten (isCommentModeratorAdmin flag)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]