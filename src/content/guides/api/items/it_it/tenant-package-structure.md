Il `TenantPackage` definisce le informazioni sul pacchetto disponibili per un `Tenant`. Un tenant può avere molti pacchetti disponibili, ma solo
uno in uso in un dato momento.

Un `Tenant` non può essere utilizzato per alcun prodotto finché il suo `packageId` non punta a un `TenantPackage` valido.

Esistono due tipi di oggetti `TenantPackage`:

1. Pacchetti a prezzo fisso - dove `hasFlexPricing` è false.
2. Prezzi flessibili - dove `hasFlexPricing` è true.

In entrambi i casi i limiti sono definiti sull'account che utilizza il pacchetto, tuttavia con Flex il tenant viene addebitato un prezzo base più
l'utilizzo effettivo, definito dai parametri `flex*`.

Un tenant può avere più tenant package e può cambiare il pacchetto da solo dalla [Pagina delle informazioni di fatturazione.](https://fastcomments.com/auth/my-account/billing-info)

Se gestirete voi stessi la fatturazione per i tenant, dovrete comunque definire un pacchetto per ciascun tenant per definire i loro limiti. Impostate semplicemente `billingHandledExternally` su `true` nel `Tenant` e non potranno modificare autonomamente le loro informazioni di fatturazione o il pacchetto attivo.

Non è possibile creare pacchetti con limiti superiori rispetto al tenant padre.

La struttura dell'oggetto `TenantPackage` è la seguente:

[inline-code-attrs-start title = 'Struttura di TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // Costo per utente SSO normale (senza permessi admin/moderatore)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Costo per utente SSO con permessi admin (flag isAccountOwner o isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Costo per utente SSO con permessi moderatore (flag isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]