El `TenantPackage` define la información del paquete disponible para un `Tenant`. Un inquilino puede tener muchos paquetes disponibles, pero solo
uno en uso en un momento dado.

Un `Tenant` no puede ser utilizado para ningún producto hasta que su `packageId` apunte a un `TenantPackage` válido.

Hay dos tipos de objetos `TenantPackage`:

1. Paquetes de precio fijo - donde `hasFlexPricing` es falso.
2. Precios flexibles - donde `hasFlexPricing` es verdadero.

En ambos casos se definen límites en la cuenta que usa el paquete, sin embargo con Flex el inquilino paga un precio base más
lo que usó, definido por los parámetros `flex*`.

Un inquilino puede tener múltiples paquetes de inquilino y tener la capacidad de cambiar el paquete ellos mismos desde la [Página de Información de Facturación.](https://fastcomments.com/auth/my-account/billing-info)

Si usted manejará la facturación de los inquilinos por su cuenta, aún necesitará definir un paquete para cada inquilino para definir sus límites. Simplemente establezca `billingHandledExternally` en `true` en el `Tenant` y ellos
no podrán cambiar su información de facturación, o paquete activo, por sí mismos.

No puede crear paquetes con límites más altos que el inquilino padre.

La estructura del objeto `TenantPackage` es la siguiente:

[inline-code-attrs-start title = 'Estructura de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // Costo por usuario SSO regular (sin permisos de admin/moderador)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Costo por usuario SSO con permisos de administrador (flags isAccountOwner o isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Costo por usuario SSO con permisos de moderador (flag isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]
