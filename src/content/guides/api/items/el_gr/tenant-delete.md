[api-resource-header-start name = 'Tenant'; route = 'DELETE /api/v1/tenants/:id'; creditsCost = 1000; api-resource-header-end]

Αυτή η διαδρομή παρέχει την αφαίρεση ενός `Tenant` **και όλων των σχετικών δεδομένων** (χρήστες, σχόλια, κλπ) με βάση το id.

Οι ακόλουθοι περιορισμοί υπάρχουν γύρω από την αφαίρεση ενοικιαστών:

- Ο ενοικιαστής πρέπει να είναι δικός σας, ή ένας ενοικιαστής white label που διαχειρίζεστε.
- Η παράμετρος ερωτήματος `sure` πρέπει να οριστεί σε `true`.

[inline-code-attrs-start title = 'Αφαίρεση Tenant cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αφαίρεσης Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Αφαίρεσης Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'unsure'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
