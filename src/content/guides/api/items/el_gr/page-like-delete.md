[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Αφαιρεί το like του τρέχοντος χρήστη από μια σελίδα. Εάν ο χρήστης δεν είχε κάνει like στη σελίδα, το αίτημα ολοκληρώνεται επιτυχώς με τον κωδικό `not-liked` και δεν αλλάζει τον αριθμό.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Απομάκρυνση Σελίδας'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Απομάκρυνσης Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** JSON κωδικοποιημένο σε URI του αντικειμένου SSO σας. Παραλείψτε για ανώνυμους χρήστες. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Απομάκρυνσης Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' όταν ο χρήστης δεν είχε κάνει like στη σελίδα. Διαφορετικά περιλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'not-liked' | string
    /** Περιλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
}
[inline-code-end]