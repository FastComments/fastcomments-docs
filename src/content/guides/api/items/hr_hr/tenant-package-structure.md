Objekt `TenantPackage` definira informacije o paketu dostupne `Tenant`u. Najamnik (tenant) može imati više dostupnih paketa, ali samo
jedan u upotrebi u datom trenutku.

`Tenant` se ne može koristiti ni za jedan proizvod dok njegov `packageId` ne pokazuje na valjani `TenantPackage`.

Postoje dvije vrste `TenantPackage` objekata:

1. Paketi s fiksnim cijenama - gdje je `hasFlexPricing` false.
2. Fleksibilno određivanje cijena - gdje je `hasFlexPricing` true.

U oba slučaja ograničenja su definirana na računu koji koristi paket, no uz Flex najamnik se naplaćuje osnovna cijena plus
ono što je potrošio, što definiraju `flex*` parametri.

Najamnik može imati više tenant paketa i može sam promijeniti paket sa [stranice s podacima za naplatu.](https://fastcomments.com/auth/my-account/billing-info)

Ako ćete sami obrađivati naplatu za najamnike, i dalje ćete morati definirati paket za svakog najamnika kako biste definirali njegova ograničenja. Jednostavno postavite `billingHandledExternally` na `true` na `Tenant`u i oni
neće moći sami mijenjati svoje podatke za naplatu niti aktivni paket.

Ne možete stvarati pakete s višim ograničenjima od nadređenog najamnika.

Struktura objekta `TenantPackage` je sljedeća:

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
    flexSSOUserCostCents?: null // Cijena po redovnom SSO korisniku (bez admin/moderator ovlasti)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Cijena po SSO korisniku s admin ovlastima (isAccountOwner ili isAdminAdmin zastavice)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Cijena po SSO korisniku s moderator ovlastima (isCommentModeratorAdmin zastavica)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]