O `TenantPackage` define as informações de pacote disponíveis para um `Tenant`. Um tenant pode ter vários pacotes disponíveis, mas apenas
um em uso em um determinado momento.

Um `Tenant` não pode ser usado para nenhum produto até que seu `packageId` aponte para um `TenantPackage` válido.

Existem dois tipos de objetos `TenantPackage`:

1. Pacotes de preço fixo - onde `hasFlexPricing` é false.
2. Preço flexível - onde `hasFlexPricing` é true.

Em ambos os casos, os limites são definidos na conta que usa o pacote, porém com Flex o tenant é cobrado por um preço base mais
o que utilizou, definido pelos parâmetros `flex*`.

Um tenant pode ter vários pacotes de tenant e ter a capacidade de alterar o pacote por conta própria na [Página de Informações de Cobrança.](https://fastcomments.com/auth/my-account/billing-info)

Se você for lidar com a cobrança dos tenants por conta própria, ainda será necessário definir um pacote para cada tenant para definir seus limites. Basta definir `billingHandledExternally` como `true` no `Tenant` e eles
não poderão alterar suas informações de cobrança, ou o pacote ativo, por conta própria.

Você não pode criar pacotes com limites superiores aos do tenant pai.

A estrutura do objeto `TenantPackage` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // Custo por usuário SSO regular (sem permissões de admin/moderador)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Custo por usuário SSO com permissões de admin (flags isAccountOwner ou isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Custo por usuário SSO com permissões de moderador (flag isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]