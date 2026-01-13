[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Αυτό το API χρησιμοποιεί σελιδοποίηση, που παρέχεται από την παράμετρο ερωτήματος `skip`. Οι TenantUsers επιστρέφονται σε σελίδες των `100`, ταξινομημένοι κατά `signUpDate`, `username` και `id`.

Το κόστος βασίζεται στον αριθμό των χρηστών ενοικιαστή που επιστρέφονται, κοστίζοντας `1 πίστωση ανά 10` χρήστες ενοικιαστή που επιστρέφονται.

[inline-code-attrs-start title = 'TenantUser cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenant users to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]
