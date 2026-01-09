The `TenantPackage` definira informacije o paketu dostupnom za `Tenant`. Tenant može imati mnogo dostupnih paketa, ali je u upotrebi samo jedan u određenom trenutku.

A `Tenant` se ne može koristiti za bilo koje proizvode dok njegov `packageId` ne pokazuje na valjani `TenantPackage`.

Postoje dvije vrste objekata `TenantPackage`:

1. Fixed-pricing packages - where `hasFlexPricing` is false.
2. Flexible pricing - where `hasFlexPricing` is true.

U oba slučaja ograničenja su definirana na računu koji koristi paket, međutim kod Flex-a tenant se naplaćuje osnovna cijena plus ono što je koristio, definirano parametrima `flex*`.

`Tenant` može imati više tenant paketa i mogućnost promjene paketa sami sa [Stranica s informacijama o naplati.](https://fastcomments.com/auth/my-account/billing-info)

Ako ćete sami voditi naplatu za tenante, i dalje ćete morati definirati paket za svaki tenant kako biste definirali njihova ograničenja. Jednostavno postavite `billingHandledExternally` na `true` na `Tenant` i oni neće moći sami mijenjati svoje podatke za naplatu ili aktivni paket.

Ne smijete kreirati pakete s većim ograničenjima nego što ima roditeljski tenant.

The structure for the `TenantPackage` object is as follows:

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
    flexSSOUserCostCents?: null // Trošak po običnom SSO korisniku (bez administratorskih/moderatorskih dozvola)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Trošak po SSO korisniku s administratorskim pravima (zastavice isAccountOwner ili isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Trošak po SSO korisniku s moderatorskim pravima (zastavica isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]