[api-resource-header-start name = 'TenantPackage'; route = 'POST /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Questa route consente di aggiungere un singolo `TenantPackage`.

La creazione di un `TenantPackage` ha le seguenti restrizioni:

- I seguenti parametri sono obbligatori:
    - `name`
    - `tenantId`
    - `monthlyCostUSD` - Può essere null.
    - `yearlyCostUSD` - Può essere null.
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
    - `hasFlexPricing` - Se `hasFlexPricing` è true, allora tutti i parametri `flex*` sono obbligatori.
- Il `name` non può essere più lungo di `50 characters`.
- Ogni elemento `forWhoText` non può essere più lungo di `200 characters`.
- Ogni elemento `featureTaglines` non può essere più lungo di `100 characters`.
- Il `TenantPackage` deve essere "smaller" rispetto al tenant padre. Ad esempio, tutti i parametri `max*` devono avere valori inferiori rispetto al tenant padre. 
- Un tenant con white labeling può avere **al massimo cinque pacchetti**.
- Solo i tenant con accesso al white labeling possono creare un `TenantPackage`.
- Non puoi aggiungere pacchetti al tuo tenant. :)

Possiamo creare un `TenantPackage` come segue:

[inline-code-attrs-start title = 'Esempio cURL di creazione TenantPackage via Email'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struttura della richiesta di creazione TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta alla creazione di TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'
    /** Incluso in caso di fallimento. **/
    reason?: string
    tenantPackage?: TenantPackage; // Restituiamo il TenantPackage completo creato in caso di successo.
}
[inline-code-end]