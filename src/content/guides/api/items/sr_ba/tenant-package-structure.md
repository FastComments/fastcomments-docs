The `TenantPackage` definiše informacije o paketu dostupnom `Tenant`-u. Tenant može imati mnogo dostupnih paketa, ali se samo jedan može koristiti u datom trenutku.

A `Tenant` se ne može koristiti za nikakve proizvode dok njegov `packageId` ne pokazuje na važeći `TenantPackage`.

Postoje dvije vrste `TenantPackage` objekata:

1. Paketi sa fiksnom cijenom - gdje je `hasFlexPricing` false.
2. Fleksibilno određivanje cijene - gdje je `hasFlexPricing` true.

U oba slučaja ograničenja su definirana na nalogu koji koristi paket, međutim kod Flex paketa tenantu se naplaćuje osnovna cijena plus ono što su koristili, definirano `flex*` parametrima.

Tenant može imati više tenant paketa i može sam promijeniti paket sa [Stranice za informacije o naplati.](https://fastcomments.com/auth/my-account/billing-info)

Ako ćete sami upravljati naplatom za tenante, i dalje ćete morati definirati paket za svakog tenanta kako biste odredili njihova ograničenja. Jednostavno postavite `billingHandledExternally` na `true` na `Tenant` i oni neće moći sami mijenjati svoje informacije o naplati ili aktivni paket.

Ne smijete kreirati pakete sa većim ograničenjima od roditeljskog tenanta.

Struktura `TenantPackage` objekta je sljedeća:

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
    flexSSOUserCostCents?: null // Trošak po običnom SSO korisniku (bez administratorskih/moderatorskih ovlaštenja)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Trošak po SSO korisniku sa administratorskim ovlaštenjima (oznake isAccountOwner ili isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Trošak po SSO korisniku sa moderatorskim ovlaštenjima (oznaka isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]