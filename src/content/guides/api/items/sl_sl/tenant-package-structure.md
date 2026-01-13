The `TenantPackage` določa informacije o paketu, ki so na voljo `Tenant`-u. `Tenant` lahko ima na voljo več paketov, vendar je hkrati lahko v uporabi le en.

`Tenant` ne more uporabljati nobenih izdelkov, dokler njegov `packageId` ne kaže na veljaven `TenantPackage`.

Obstajata dve vrsti objektov `TenantPackage`:

1. Paketi s fiksno ceno - kjer je `hasFlexPricing` false.
2. Fleksibilno pricevanje - kjer je `hasFlexPricing` true.

V obeh primerih so omejitve določene na računu, ki uporablja paket, vendar pri Flex najemnika zaračunamo osnovno ceno plus porabo, določeno z `flex*` parametri.

`Tenant` lahko ima več paketov in lahko spremeni paket sam na [Billing Info Page.](https://fastcomments.com/auth/my-account/billing-info)

Če boste sami urejali obračunavanje za najemnike, boste še vedno morali določiti paket za vsakega najemnika, da definirate njihove omejitve. Preprosto nastavite `billingHandledExternally` na `true` na `Tenant` in ne bodo mogli sami spreminjati svojih podatkov o obračunavanju ali aktivnega paketa.

Ne smete ustvarjati paketov z višjimi omejitvami kot ima nadrejeni tenant.

Struktura objekta `TenantPackage` je naslednja:

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
    flexSSOUserCostCents?: null // Strošek na običajnega SSO uporabnika (brez administratorskih/moderatorskih dovoljenj)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Strošek na SSO uporabnika z administratorskimi dovoljenji (flagi isAccountOwner ali isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Strošek na SSO uporabnika z moderatorskimi dovoljenji (z zastavico isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]