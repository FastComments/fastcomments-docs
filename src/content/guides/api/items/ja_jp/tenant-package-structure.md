`TenantPackage` は、`Tenant` が利用できるパッケージ情報を定義します。テナントは複数のパッケージを利用可能にすることができますが、特定の時点で使用できるのは一つだけです。

`packageId` が有効な `TenantPackage` を指していない限り、`Tenant` はいかなる製品にも使用できません。

`TenantPackage` オブジェクトには次の2種類があります:

1. 固定価格のパッケージ - `hasFlexPricing` が false の場合。
2. 柔軟価格のパッケージ - `hasFlexPricing` が true の場合。

どちらの場合も、パッケージを使用するアカウントに対して制限が定義されますが、Flex の場合はテナントはベース価格に加えて使用量に応じて課金されます。使用量は `flex*` パラメータで定義されます。

テナントは複数のテナントパッケージを持つことができ、[請求情報ページ](https://fastcomments.com/auth/my-account/billing-info) から自分でパッケージを変更することができます。

テナントの請求をあなたが代行して処理する場合でも、各テナントの制限を定義するためにパッケージを定義する必要があります。`Tenant` の `billingHandledExternally` を `true` に設定するだけで、テナントは自分で請求情報や有効なパッケージを変更できなくなります。

親テナントより高い制限を持つパッケージを作成することはできません。

`TenantPackage` オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'TenantPackage の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // 管理者／モデレーター権限のない通常のSSOユーザーあたりのコスト
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // 管理者権限を持つSSOユーザーあたりのコスト（isAccountOwner または isAdminAdmin フラグ）
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // モデレーター権限を持つSSOユーザーあたりのコスト（isCommentModeratorAdmin フラグ）
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]