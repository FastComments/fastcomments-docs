[api-resource-header-start name = 'TenantPackage'; route = 'POST /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή παρέχει τη δυνατότητα προσθήκης ενός μόνου `TenantPackage`.

Η δημιουργία ενός `TenantPackage` έχει τους ακόλουθους περιορισμούς:

- Οι ακόλουθες παράμετροι είναι απαιτούμενες:
    - `name`
    - `tenantId`
    - `monthlyCostUSD` - Μπορεί να είναι null.
    - `yearlyCostUSD` - Μπορεί να είναι null.
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
    - `hasFlexPricing` - Αν είναι true, τότε όλες οι παράμετροι `flex*` απαιτούνται.
- Το `name` δεν μπορεί να είναι μεγαλύτερο από `50 χαρακτήρες`.
- Κάθε στοιχείο `forWhoText` δεν μπορεί να είναι μεγαλύτερο από `200 χαρακτήρες`.
- Κάθε στοιχείο `featureTaglines` δεν μπορεί να είναι μεγαλύτερο από `100 χαρακτήρες`.
- Το `TenantPackage` πρέπει να είναι "μικρότερο" από τον γονικό ενοικιαστή. Για παράδειγμα, όλες οι παράμετροι `max*` πρέπει να έχουν χαμηλότερες τιμές από τον γονικό ενοικιαστή.
- Ένας white labeled ενοικιαστής μπορεί να έχει **μέχρι πέντε πακέτα**.
- Μόνο οι ενοικιαστές με πρόσβαση white labeling μπορούν να δημιουργήσουν ένα `TenantPackage`.
- Δεν μπορείτε να προσθέσετε πακέτα στον δικό σας ενοικιαστή. :)

Μπορούμε να δημιουργήσουμε ένα `TenantPackage` ως εξής:

[inline-code-attrs-start title = 'Δημιουργία TenantPackage μέσω Email cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Δομή Αιτήματος Δημιουργίας TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Δημιουργίας TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
