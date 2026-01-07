[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα ενημέρωσης ενός `TenantPackage` με βάση το `id`.

Η ενημέρωση ενός `TenantPackage` έχει τους ακόλουθους περιορισμούς:

- Αν ορίζετε το `hasFlexPricing` σε true, τότε όλες οι παράμετροι `flex*` απαιτούνται στο ίδιο αίτημα.
- Το `name` δεν μπορεί να είναι μεγαλύτερο από `50 χαρακτήρες`.
- Κάθε στοιχείο `forWhoText` δεν μπορεί να είναι μεγαλύτερο από `200 χαρακτήρες`.
- Κάθε στοιχείο `featureTaglines` δεν μπορεί να είναι μεγαλύτερο από `100 χαρακτήρες`.
- Το `TenantPackage` πρέπει να είναι "μικρότερο" από τον γονικό ενοικιαστή. Για παράδειγμα, όλες οι παράμετροι `max*` πρέπει να έχουν χαμηλότερες τιμές από τον γονικό ενοικιαστή.
- Δεν μπορείτε να αλλάξετε το `tenantId` που συσχετίζεται με ένα `TenantPackage`.

[inline-code-attrs-start title = 'TenantPackage PATCH cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος TenantPackage PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης TenantPackage PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
