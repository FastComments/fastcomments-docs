[api-resource-header-start name = 'TenantPackage'; route = 'POST /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Diese Route bietet die Möglichkeit, ein einzelnes `TenantPackage` hinzuzufügen.

Das Erstellen eines `TenantPackage` hat die folgenden Einschränkungen:

- Die folgenden Parameter sind erforderlich:
    - `name`
    - `tenantId`
    - `monthlyCostUSD` - Kann null sein.
    - `yearlyCostUSD` - Kann null sein.
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
    - `hasFlexPricing` - Wenn true, sind alle `flex*`-Parameter erforderlich.
- Der `name` darf nicht länger als `50 Zeichen` sein.
- Jedes `forWhoText`-Element darf nicht länger als `200 Zeichen` sein.
- Jedes `featureTaglines`-Element darf nicht länger als `100 Zeichen` sein.
- Das `TenantPackage` muss "kleiner" als der übergeordnete Tenant sein. Beispielsweise müssen alle `max*`-Parameter niedrigere Werte als der übergeordnete Tenant haben.
- Ein White-Label-Tenant darf **maximal fünf Pakete** haben.
- Nur Tenants mit White-Labeling-Zugang können ein `TenantPackage` erstellen.
- Sie können keine Pakete zu Ihrem eigenen Tenant hinzufügen. :)

Wir können ein `TenantPackage` wie folgt erstellen:

[inline-code-attrs-start title = 'TenantPackage Erstellen per E-Mail cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'TenantPackage Erstellen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Erstellen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
