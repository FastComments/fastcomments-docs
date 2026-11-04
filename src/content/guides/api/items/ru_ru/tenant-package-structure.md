`TenantPackage` определяет информацию о пакете, доступном для `Tenant`. У тенанта может быть доступно несколько пакетов, но только
один используется в данный момент.

`Tenant` нельзя использовать ни для каких продуктов, пока его `packageId` не указывает на действительный `TenantPackage`.

Существует два типа объектов `TenantPackage`:

1. Пакеты с фиксированной ценой — где `hasFlexPricing` равно false.
2. Гибкая цена — где `hasFlexPricing` равно true.

В обоих случаях лимиты определяются на аккаунте, использующем пакет, однако при Flex-тарифе с тенанта взимается базовая цена плюс
то, что он использовал, что определяется параметрами `flex*`.

У тенанта может быть несколько пакетов, и он может сам менять пакет на [странице информации о биллинге.](https://fastcomments.com/auth/my-account/billing-info)

Если вы сами будете заниматься биллингом для тенантов, вам всё равно нужно будет определить пакет для каждого тенанта, чтобы задать их лимиты. Просто установите `billingHandledExternally` в `true` для `Tenant`, и они
не смогут самостоятельно менять свою платежную информацию или активный пакет.

Вы не можете создавать пакеты с лимитами выше, чем у родительского тенанта.

Структура объекта `TenantPackage` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // Стоимость за обычного SSO-пользователя (без прав администратора/модератора)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Стоимость за SSO-пользователя с правами администратора (флаги isAccountOwner или isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Стоимость за SSO-пользователя с правами модератора (флаг isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]