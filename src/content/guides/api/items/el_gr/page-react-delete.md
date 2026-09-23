[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Αφαιρεί μία από τις αντιδράσεις του τρέχοντος χρήστη από μια σελίδα. Εάν ο χρήστης δεν έχει προσθέσει αυτήν την αντίδραση, το αίτημα ολοκληρώνεται με επιτυχία με τον κωδικό `no-react` και δεν αλλάζει τον αριθμό.

[inline-code-attrs-start title = 'Παράδειγμα cURL Διαγραφής Αντίδρασης Σελίδας'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Διαγραφής Αντίδρασης Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** Το αναγνωριστικό της αντίδρασης. **/
    id: string
    /** JSON κωδικοποιημένο σε URI του αντικειμένου SSO. Παραλείψτε για ανώνυμους χρήστες. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Διαγραφής Αντίδρασης Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' όταν ο χρήστης δεν είχε προσθέσει αυτήν την αντίδραση. Διαφορετικά περιλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'no-react' | string
    /** Περιλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
}
[inline-code-end]

---