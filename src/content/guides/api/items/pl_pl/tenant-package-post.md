[api-resource-header-start name = 'TenantPackage'; route = 'POST /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Ta trasa umożliwia dodanie pojedynczego `TenantPackage`.

Tworzenie `TenantPackage` ma następujące ograniczenia:

- Następujące parametry są wymagane:
    - `name`
    - `tenantId`
    - `monthlyCostUSD` - Może być null.
    - `yearlyCostUSD` - Może być null.
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
    - `hasFlexPricing` - Jeśli true, wtedy wszystkie parametry `flex*` są wymagane.
- `name` nie może być dłuższe niż `50 characters`.
- Każdy element `forWhoText` nie może być dłuższy niż `200 characters`.
- Każdy element `featureTaglines` nie może być dłuższy niż `100 characters`.
- `TenantPackage` musi być "mniejszy" niż nadrzędny tenant. Na przykład wszystkie parametry `max*` muszą mieć niższe wartości niż nadrzędny tenant.
- Najemca z white-labelingiem może mieć maksymalnie **pięć pakietów**.
- Tylko tenanci z dostępem do white labelingu mogą tworzyć `TenantPackage`.
- Nie możesz dodawać pakietów do własnego najemcy. :)

Możemy utworzyć `TenantPackage` w następujący sposób:

[inline-code-attrs-start title = 'Przykład cURL tworzenia TenantPackage przez Email'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura żądania tworzenia TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi tworzenia TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    tenantPackage?: TenantPackage; // Zwracamy kompletny utworzony tenantPackage w przypadku powodzenia.
}
[inline-code-end]