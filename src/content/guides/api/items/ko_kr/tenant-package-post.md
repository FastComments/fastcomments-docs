[api-resource-header-start name = 'TenantPackage'; route = 'POST /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

이 라우트는 단일 `TenantPackage`를 추가할 수 있는 기능을 제공합니다.

`TenantPackage` 생성에는 다음과 같은 제한 사항이 있습니다:

- 다음 매개변수가 필요합니다:
    - `name`
    - `tenantId`
    - `monthlyCostUSD` - null일 수 있습니다.
    - `yearlyCostUSD` - null일 수 있습니다.
    - `maxMonthlyPageLoads`
    - `maxMonthlyAPICredits`
    - `maxMonthlyComments`
    - `maxConcurrentUsers`
    - `maxTenantUsers`
    - `maxSSOUsers`
    - `maxModerators`
    - `maxDomains`
    - `hasDebranding`
    - `forWhoText`
    - `featureTaglines`
    - `hasFlexPricing` - true이면 모든 `flex*` 매개변수가 필요합니다.
- `name`은 `50 characters`보다 길 수 없습니다.
- 각 `forWhoText` 항목은 `200 characters`보다 길 수 없습니다.
- 각 `featureTaglines` 항목은 `100 characters`보다 길 수 없습니다.
- `TenantPackage`는 부모 테넌트보다 "작아야" 합니다. 예를 들어 모든 `max*` 매개변수는 부모 테넌트보다 낮은 값을 가져야 합니다. 
- 화이트 라벨링된 테넌트는 **최대 다섯 개의 패키지**를 가질 수 있습니다.
- 화이트 라벨링 접근 권한이 있는 테넌트만 `TenantPackage`를 생성할 수 있습니다.
- 자신의 테넌트에는 패키지를 추가할 수 없습니다. :)

다음과 같이 `TenantPackage`를 생성할 수 있습니다:

[inline-code-attrs-start title = 'TenantPackage 생성 - 이메일 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
  "name": "Default Package",
  "tenantId": "some-child-tenant-id",
  "monthlyCostUSD": null,
  "yearlyCostUSD": null,
  "maxMonthlyPageLoads": 50000,
  "maxMonthlyAPICredits": 50000,
  "maxMonthlyComments": 50000,
  "maxConcurrentUsers": 50000,
  "maxTenantUsers": 10,
  "maxSSOUsers": 50000,
  "maxModerators": 100,
  "maxDomains": 3,
  "hasWhiteLabeling": false,
  "hasDebranding": true,
  "forWhoText": "For Everyone",
  "featureTaglines": [
    "Some Tag",
    "Some Other Tag"
  ],
  "hasFlexPricing": true,
  "flexPageLoadCostCents": 100,
  "flexPageLoadUnit": 100000,
  "flexCommentCostCents": 100,
  "flexCommentUnit": 100000,
  "flexSSOUserCostCents": 100,
  "flexSSOUserUnit": 1000,
  "flexAPICreditCostCents": 100,
  "flexAPICreditUnit": 50000,
  "flexModeratorCostCents": 500,
  "flexModeratorUnit": 1,
  "flexAdminCostCents": 1000,
  "flexAdminUnit": 1,
  "flexDomainCostCents": 1000,
  "flexDomainUnit": 1,
  "flexMinimumCostCents": 99
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 생성 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 생성 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'
    /** 실패 시 포함됩니다. **/
    reason?: string
    tenantPackage?: TenantPackage; // 성공 시 생성된 전체 테넌트 패키지를 반환합니다.
}
[inline-code-end]