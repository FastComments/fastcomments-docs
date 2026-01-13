Le `TenantPackage` définit les informations de forfait disponibles pour un `Tenant`. Un locataire peut avoir plusieurs forfaits disponibles, mais un seul
en utilisation à un moment donné.

Un `Tenant` ne peut être utilisé pour aucun produit tant que son `packageId` ne pointe pas vers un `TenantPackage` valide.

Il existe deux types d'objets `TenantPackage` :

1. Forfaits à prix fixe - où `hasFlexPricing` est faux.
2. Tarification flexible - où `hasFlexPricing` est vrai.

Dans les deux cas, des limites sont définies sur le compte utilisant le forfait, cependant avec Flex, le locataire est facturé un prix de base plus
ce qu'il a utilisé, défini par les paramètres `flex*`.

Un locataire peut avoir plusieurs forfaits de locataire et avoir la possibilité de changer le forfait lui-même depuis la [Page d'Information de Facturation.](https://fastcomments.com/auth/my-account/billing-info)

Si vous gérerez la facturation pour les locataires vous-mêmes, vous devrez toujours définir un forfait pour chaque locataire pour définir leurs limites. Définissez simplement `billingHandledExternally` à `true` sur le `Tenant` et ils
ne pourront pas changer leurs informations de facturation, ou forfait actif, eux-mêmes.

Vous ne pouvez pas créer de forfaits avec des limites plus élevées que le locataire parent.

La structure de l'objet `TenantPackage` est la suivante :

[inline-code-attrs-start title = 'Structure de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
