[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Επιστρέφει τα ονόματα των χρηστών που πρόσθεσαν μια αντίδραση σε μια σελίδα, ταξινομημένα αλφαβητικά. Αναζητούνται έως 100 αντιδράσεις, και οι ανώνυμοι χρήστες δεν περιλαμβάνονται.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Χρήστες Σελίδας React'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Χρηστών Σελίδας React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Χρηστών Σελίδας React'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]