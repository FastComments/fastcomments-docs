`TenantPackage` definerer pakkeinformation tilgængelig for en `Tenant`. En tenant kan have mange pakker tilgængelige, men kun
én i brug på et givent tidspunkt.

En `Tenant` kan ikke bruges til nogen produkter, før dens `packageId` peger på en gyldig `TenantPackage`.

Der er to typer `TenantPackage`-objekter:

1. Fastpris-pakker - hvor `hasFlexPricing` er false.
2. Fleksibel prissætning - hvor `hasFlexPricing` er true.

I begge tilfælde defineres grænser på kontoen ved hjælp af pakken, dog med Flex opkræves tenant en basispris plus
hvad de brugte, defineret af `flex*`-parametrene.

En tenant kan have flere tenant-pakker og have mulighed for selv at ændre pakken fra [Faktureringsinformationssiden.](https://fastcomments.com/auth/my-account/billing-info)

Hvis du selv vil håndtere fakturering for tenants, skal du stadig definere en pakke for hver tenant for at definere deres grænser. Sæt blot `billingHandledExternally` til `true` på `Tenant`, og de
vil ikke kunne ændre deres faktureringsinformation eller aktive pakke selv.

Du kan ikke oprette pakker med højere grænser end den overordnede tenant.

Strukturen for `TenantPackage`-objektet er som følger:

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
