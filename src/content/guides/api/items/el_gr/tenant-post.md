[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή παρέχει τη δυνατότητα προσθήκης ενός μόνου `Tenant`.

Η δημιουργία ενός `Tenant` έχει τους ακόλουθους περιορισμούς:

- Ένα `name` είναι απαιτούμενο.
- Το `domainConfiguration` είναι απαιτούμενο.
- Οι ακόλουθες τιμές δεν μπορούν να παρέχονται κατά τη δημιουργία ενός `Tenant`:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
- Η `signUpDate` δεν μπορεί να είναι στο μέλλον.
- Το `name` δεν μπορεί να είναι μεγαλύτερο από `200 χαρακτήρες`.
- Το `email` δεν μπορεί να είναι μεγαλύτερο από `300 χαρακτήρες`.
- Το `email` πρέπει να είναι μοναδικό σε όλους τους ενοικιαστές του FastComments.com.
- Δεν μπορείτε να δημιουργήσετε ενοικιαστές αν ο γονικός ενοικιαστής δεν έχει ένα έγκυρο `TenantPackage` ορισμένο.
  - Αν ο ενοικιαστής σας δημιουργήθηκε μέσω του FastComments.com, αυτό δεν θα πρέπει να είναι πρόβλημα.
- Δεν μπορείτε να δημιουργήσετε περισσότερους ενοικιαστές από αυτούς που ορίζονται στο `maxWhiteLabeledTenants` στο πακέτο σας.
- Πρέπει να καθορίσετε την παράμετρο ερωτήματος `tenantId` που είναι το id του `γονικού ενοικιαστή` σας με ενεργοποιημένο το white labeling.

Μπορούμε να δημιουργήσουμε έναν `Tenant` με μόνο λίγες παραμέτρους:

[inline-code-attrs-start title = 'Δημιουργία Tenant cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "domainConfiguration": [ { "domain": "somedomain.com" } ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Δημιουργίας Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Δημιουργίας Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Included on failure. **/
    reason?: string
    tenant?: Tenant; // We return the complete created tenant on success.
}
[inline-code-end]
