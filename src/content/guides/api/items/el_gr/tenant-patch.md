[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

Αυτό το API endpoint παρέχει τη δυνατότητα ενημέρωσης ενός `Tenant` με βάση το `id`.

Η ενημέρωση ενός `Tenant` έχει τους ακόλουθους περιορισμούς:

- Οι ακόλουθες τιμές δεν μπορούν να ενημερωθούν:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
  - `managedByTenantId`
- Η `signUpDate` δεν μπορεί να είναι στο μέλλον.
- Το `name` δεν μπορεί να είναι μεγαλύτερο από `200 χαρακτήρες`.
- Το `email` δεν μπορεί να είναι μεγαλύτερο από `300 χαρακτήρες`.
- Το `email` πρέπει να είναι μοναδικό σε όλους τους ενοικιαστές του FastComments.com.
- Όταν ορίζετε το `billingInfoValid` σε `true`, το `billingInfo` πρέπει να παρέχεται στο ίδιο αίτημα.
- Δεν μπορείτε να ενημερώσετε το `packageId` που συσχετίζεται με τον δικό σας ενοικιαστή.
- Δεν μπορείτε να ενημερώσετε το `paymentFrequency` που συσχετίζεται με τον δικό σας ενοικιαστή.

[inline-code-attrs-start title = 'Tenant PATCH cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Tenant PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Tenant PATCH'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
