The `TenantPackage` визначає інформацію про пакет, доступний для `Tenant`. У одного tenant може бути багато доступних пакетів, але лише
один використовується в певний момент часу.

З `Tenant` не можна користуватися жодними продуктами, поки його `packageId` не вказує на дійсний `TenantPackage`.

Існує два типи об'єктів `TenantPackage`:

1. Пакети з фіксованими тарифами - де `hasFlexPricing` має значення false.
2. Гнучке ціноутворення - де `hasFlexPricing` має значення true.

У обох випадках ліміти визначаються для облікового запису, що використовує пакет, проте у випадку Flex орендарю нараховується базова плата плюс оплата за фактичне використання, яка задається параметрами `flex*`.

У tenant може бути кілька tenant пакетів, і він може самостійно змінювати активний пакет на [Сторінці платіжної інформації.](https://fastcomments.com/auth/my-account/billing-info)

Якщо ви будете самостійно обробляти білінг для tenant-ів, вам все одно потрібно визначити пакет для кожного tenant, щоб задати їхні ліміти. Просто встановіть `billingHandledExternally` в `true` на `Tenant`, і вони не зможуть змінювати свою платіжну інформацію або активний пакет самостійно.

Ви не можете створювати пакети з лімітами вищими за ліміти батьківського tenant.

Структура об'єкта `TenantPackage` виглядає наступним чином:

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
    flexSSOUserCostCents?: null // Вартість за звичайного SSO-користувача (без прав адміністратора/модератора)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Вартість за SSO-користувача з правами адміністратора (прапори isAccountOwner або isAdminAdmin)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Вартість за SSO-користувача з правами модератора (прапор isCommentModeratorAdmin)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]

---