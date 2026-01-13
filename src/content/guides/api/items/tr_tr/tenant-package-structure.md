`TenantPackage`, bir `Tenant` için mevcut paket bilgilerini tanımlar. Bir tenant'ın birden çok paketi olabilir, ancak aynı anda yalnızca biri kullanımda olabilir.

Bir `Tenant`, `packageId` geçerli bir `TenantPackage`'ı işaret edene kadar herhangi bir ürün için kullanılamaz.

İki tür `TenantPackage` nesnesi vardır:

1. Sabit fiyatlı paketler - `hasFlexPricing` false olduğunda.
2. Esnek fiyatlandırma - `hasFlexPricing` true olduğunda.

Her iki durumda da paketi kullanan hesap üzerinde limitler tanımlanır, ancak Flex ile tenant'a baz bir ücret artı kullandıkları miktar `flex*` parametreleri ile tanımlanan şekilde faturalandırılır.

Bir tenant'ın birden fazla tenant paketi olabilir ve paketi kendileri [Fatura Bilgileri Sayfası.](https://fastcomments.com/auth/my-account/billing-info) üzerinden değiştirme yetkisi olabilir.

Faturalamayı tenantlar için kendiniz yönetecekseniz, limitlerini tanımlamak için yine de her tenant için bir paket tanımlamanız gerekir. `Tenant` üzerinde `billingHandledExternally`'i `true` olarak ayarlamanız yeterlidir; böylece kendi fatura bilgilerini veya aktif paketlerini değiştiremeyeceklerdir.

Üst tenant'tan daha yüksek limitlere sahip paketler oluşturamazsınız.

`TenantPackage` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'TenantPackage Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // Normal SSO kullanıcı başına maliyet (yönetici/moderatör izinleri olmadan)
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // Yönetici izinlerine sahip SSO kullanıcı başına maliyet (isAccountOwner veya isAdminAdmin bayrakları)
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // Moderatör izinlerine sahip SSO kullanıcı başına maliyet (isCommentModeratorAdmin bayrağı)
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]