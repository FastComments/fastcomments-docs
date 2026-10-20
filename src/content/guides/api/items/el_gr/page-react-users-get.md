[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Επιστρέφει τα ονόματα των χρηστών που πρόσθεσαν μια αντίδραση σε μια σελίδα, ταξινομημένα αλφαβητικά. Αναζητούνται έως 100 αντιδράσεις, και δεν περιλαμβάνονται ανώνυμοι χρήστες και χρήστες με ιδιωτική δραστηριότητα προφίλ. Σε σελίδες που απαιτούν SSO, οι χρήστες που δεν μπορούν να δουν τη σελίδα λαμβάνουν ένα `403` με τον κωδικό `sso-required` ή `page-forbidden`.

Για να λάβετε επίσης avatars και IDs χρηστών, χρησιμοποιήστε [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) με `includeUserInfo=public`.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Χρήστες Αντιδράσεων Σελίδας'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Χρηστών Αντιδράσεων Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** Το αναγνωριστικό της αντίδρασης. **/
    id: string
    /** Κωδικοποιημένο σε URI JSON του αντικειμένου SSO σας. Παραλείψτε για ανώνυμους χρήστες. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Χρηστών Αντιδράσεων Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: string
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]