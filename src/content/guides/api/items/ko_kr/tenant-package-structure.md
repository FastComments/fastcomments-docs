`TenantPackage`는 `Tenant`에 제공되는 패키지 정보를 정의합니다. 테넌트는 여러 패키지를 사용할 수 있지만, 주어진 시점에는 하나만 사용할 수 있습니다.

`Tenant`의 `packageId`가 유효한 `TenantPackage`를 가리키기 전까지는 어떤 제품에도 사용할 수 없습니다.

`TenantPackage` 객체에는 두 가지 유형이 있습니다:

1. 고정 가격 패키지 - `hasFlexPricing`이 false인 경우.
2. 유연한 가격 책정 - `hasFlexPricing`이 true인 경우.

두 경우 모두 패키지를 사용하는 계정에 한도가 정의되지만, Flex의 경우 테넌트는 기본 가격에 `flex*` 매개변수로 정의된 사용량만큼 추가로 청구됩니다.

테넌트는 여러 테넌트 패키지를 가질 수 있으며, [청구 정보 페이지](https://fastcomments.com/auth/my-account/billing-info)에서 직접 패키지를 변경할 수 있습니다.

테넌트의 청구를 직접 처리할 경우에도 각 테넌트의 한도를 정의하기 위해 패키지를 정의해야 합니다. `Tenant`에서 `billingHandledExternally`를 `true`로 설정하기만 하면, 테넌트가 직접 청구 정보나 활성 패키지를 변경할 수 없게 됩니다.

부모 테넌트보다 높은 한도를 가진 패키지는 생성할 수 없습니다.

`TenantPackage` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'TenantPackage 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    flexSSOUserCostCents?: null // 일반 SSO 사용자(관리자/중재자 권한 없음)당 비용
    flexSSOUserUnit?: null
    flexAPICreditCostCents?: null
    flexAPICreditUnit?: null
    flexModeratorCostCents?: null
    flexModeratorUnit?: null
    flexAdminCostCents?: null
    flexAdminUnit?: null
    flexDomainCostCents?: null
    flexDomainUnit?: null
    flexSSOAdminCostCents?: null // 관리자 권한(isAccountOwner 또는 isAdminAdmin 플래그)을 가진 SSO 사용자당 비용
    flexSSOAdminUnit?: null
    flexSSOModeratorCostCents?: null // 중재자 권한(isCommentModeratorAdmin 플래그)을 가진 SSO 사용자당 비용
    flexSSOModeratorUnit?: null
    flexMinimumCostCents?: null
}
[inline-code-end]