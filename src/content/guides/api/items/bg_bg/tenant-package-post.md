[api-resource-header-start name = 'TenantPackage'; route = 'POST /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Този маршрут предоставя възможност за добавяне на единичен `TenantPackage`.

Създаването на `TenantPackage` има следните ограничения:

- Следните параметри са задължителни:
    - `name`
    - `tenantId`
    - `monthlyCostUSD` - Може да бъде null.
    - `yearlyCostUSD` - Може да бъде null.
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
    - `hasFlexPricing` - Ако е true, тогава всички `flex*` параметри са задължителни.
- `name` не може да бъде по-дълго от `50 символа`.
- Всеки елемент на `forWhoText` не може да бъде по-дълъг от `200 символа`.
- Всеки елемент на `featureTaglines` не може да бъде по-дълъг от `100 символа`.
- `TenantPackage` трябва да бъде "по-малък" от родителския tenant. Например, всички `max*` параметри трябва да имат по-ниски стойности от родителския tenant.
- White labeled tenant може да има **максимум пет пакета**.
- Само tenant-и с достъп до white labeling могат да създават `TenantPackage`.
- Не можете да добавяте пакети към вашия собствен tenant. :)

Можем да създадем `TenantPackage` по следния начин:

[inline-code-attrs-start title = 'Пример за създаване на TenantPackage с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на заявката за създаване на TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за създаване на TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'
    /** Included on failure. **/
    reason?: string
    tenantPackage?: TenantPackage; // We return the complete created tenant package on success.
}
[inline-code-end]
