[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Κάνει like σε μια σελίδα ως ο τρέχων χρήστης. Κάθε χρήστης μπορεί να κάνει like σε μια σελίδα μία φορά: η επαναλαμβανόμενη ενέργεια επιτυγχάνει με τον κωδικό `already-liked` και δεν αλλάζει τον αριθμό.

Η σελίδα δημιουργείται εάν δεν υπάρχει ακόμη. Περνάτε το `title` για να ορίσετε ή να ενημερώσετε τον τίτλο της σελίδας.

[inline-code-attrs-start title = 'Παράδειγμα cURL Like Σελίδας'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Like Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Ορίζει τον τίτλο της σελίδας. **/
    title?: string
    /** Κωδικοποιημένο σε URI JSON του αντικειμένου SSO σας. Παραλείψτε για ανώνυμους χρήστες. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Like Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' όταν ο χρήστης είχε ήδη κάνει like τη σελίδα. Διαφορετικά περιλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'already-liked' | string
    /** Περιλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
}
[inline-code-end]