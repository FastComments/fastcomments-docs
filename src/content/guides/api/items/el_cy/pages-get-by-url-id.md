[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages/by-url-id'; creditsCost = 1; api-resource-header-end]

Μεμονωμένες σελίδες μπορούν να ανακτηθούν με βάση το αντίστοιχο `urlId`. Αυτό μπορεί να είναι χρήσιμο για αναζήτηση τίτλων σελίδων ή αριθμών σχολίων.

[inline-code-attrs-start title = 'Σελίδα ανά URL ID cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/by-url-id?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Σελίδας Ανά URL ID'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Σελίδας ανά URL ID'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    page?: Page[] | null
}
[inline-code-end]

#### Χρήσιμη Συμβουλή

Θυμηθείτε να κωδικοποιήσετε URI τιμές όπως το `urlId`.
