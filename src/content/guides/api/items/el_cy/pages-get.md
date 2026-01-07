[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Αυτή τη στιγμή μπορείτε να ανακτήσετε μόνο όλες τις σελίδες (ή μια μόνο σελίδα μέσω `/by-url-id`) που σχετίζονται με τον λογαριασμό σας. Αν θέλετε πιο λεπτομερή αναζήτηση, [επικοινωνήστε μαζί μας](https://fastcomments.com/auth/my-account/help).

[inline-code-attrs-start title = 'Pages cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Χρήσιμη Συμβουλή

Το API `Comment` απαιτεί ένα `urlId`. Μπορείτε να καλέσετε πρώτα το API `Pages`, για να δείτε πώς μοιάζουν οι διαθέσιμες τιμές `urlId`
για εσάς.
