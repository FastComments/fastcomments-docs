[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Αυτό το API επιστρέφει ενοικιαστές που διαχειρίζονται από τον ενοικιαστή σας.

Η σελιδοποίηση παρέχεται από την παράμετρο ερωτήματος `skip`. Οι ενοικιαστές επιστρέφονται σε σελίδες των `100`, ταξινομημένοι κατά `signUpDate` και `id`.

Το κόστος βασίζεται στον αριθμό των ενοικιαστών που επιστρέφονται, κοστίζοντας `1 πίστωση ανά 10` ενοικιαστές που επιστρέφονται.

[inline-code-attrs-start title = 'Tenant cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Μπορείτε να ορίσετε παραμέτρους `meta` στα αντικείμενα `Tenant` και να αναζητήσετε αντίστοιχους ενοικιαστές. Για παράδειγμα, για το κλειδί `someKey` και την τιμή meta `some-value`, μπορούμε να
κατασκευάσουμε ένα αντικείμενο JSON με αυτό το ζεύγος κλειδιού/τιμής και μετά να το κωδικοποιήσουμε URI ως παράμετρο ερωτήματος για φιλτράρισμα:

[inline-code-attrs-start title = 'Αναζήτηση Tenant κατά Meta cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenants to skip for pagination. **/
    skip?: number
    /** Filter by meta params. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]
