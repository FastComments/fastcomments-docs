`TenantPackage` definiše informacije o paketu dostupnom `Tenant`. Tenant može imati više dostupnih paketa, ali je u upotrebi samo jedan u datom trenutku.

`Tenant` se ne može koristiti za bilo koje proizvode dok njegov `packageId` ne ukazuje na važeći `TenantPackage`.

Postoje dve vrste `TenantPackage` objekata:

1. Paketi sa fiksnim cenama - gde je `hasFlexPricing` false.
2. Fleksibilno određivanje cena - gde je `hasFlexPricing` true.

U oba slučaja ograničenja se definišu na nalogu koji koristi paket, međutim kod Flex-a tenant se naplaćuje osnovna cena plus ono što su koristili, definisano `flex*` parametrima.

Tenant može imati više tenant paketa i mogućnost da sam promeni paket sa [Stranice sa informacijama o naplati.](https://fastcomments.com/auth/my-account/billing-info)

Ako ćete vi sami obrađivati naplatu za tenante, i dalje ćete morati da definišete paket za svakog tenanta da biste odredili njihova ograničenja. Jednostavno postavite `billingHandledExternally` na `true` na `Tenant` i oni neće moći sami da menjaju svoje podatke o naplati ili aktivni paket.

Ne smete kreirati pakete sa većim ograničenjima od roditeljskog tenanta.

Struktura `TenantPackage` objekta je sledeća:

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
    flexSSOUserCostCents?: null // Cena po običnom SSO korisniku (bez administratorskih ili moderatorskih dozvola)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Cena po SSO korisniku sa administratorskim privilegijama (oznake isAccountOwner ili isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Cena po SSO korisniku sa moderatorskim privilegijama (oznake isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]