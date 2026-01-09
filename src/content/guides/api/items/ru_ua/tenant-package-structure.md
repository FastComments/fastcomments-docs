Объект `TenantPackage` определяет информацию о пакете, доступном для `Tenant`. У `Tenant` может быть несколько доступных пакетов, но одновременно используется только один.

`Tenant` не может использоваться для каких-либо продуктов, пока его `packageId` не указывает на действительный `TenantPackage`.

Существует два типа объектов `TenantPackage`:

1. Пакеты с фиксированной ценой - где `hasFlexPricing` равно false.
2. Гибкая тарификация - где `hasFlexPricing` равно true.

В обоих случаях для аккаунта, использующего пакет, определяются ограничения, однако при Flex-ценообразовании тенант платит базовую цену плюс стоимость использованных ресурсов, определяемую параметрами `flex*`.

Тенант может иметь несколько пакетов и иметь возможность самостоятельно менять пакет со страницы [Страница платёжной информации.](https://fastcomments.com/auth/my-account/billing-info)

Если вы будете самостоятельно вести выставление счетов для тенантов, вам всё равно нужно определить пакет для каждого тенанта, чтобы задать их лимиты. Просто установите `billingHandledExternally` в `true` у `Tenant` и они не смогут самостоятельно изменять платёжную информацию или активный пакет.

Нельзя создавать пакеты с лимитами выше, чем у родительского `Tenant`.

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