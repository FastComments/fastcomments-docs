[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Επιστρέφει τον αριθμό για κάθε αντίδραση σε μια σελίδα, καθώς και ποιες αντιδράσεις έχει προσθέσει ο τρέχων χρήστης.

Περάστε `includeUserInfo=public` για να λάβετε επίσης τους χρήστες που αντέδρασαν, ως μία καταχώρηση ανά αντίδραση με το αναγνωριστικό της αντίδρασης, πιο πρόσφατες πρώτες. Οι χρήστες με ιδιωτική δραστηριότητα προφίλ και οι ανώνυμοι χρήστες εξαιρούνται, και αναζητούνται έως 100 αντιδράσεις. Μόνο `public` γίνεται αποδεκτό εδώ· `include-private` επιστρέφει `422` και είναι διαθέσιμο από [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) με κλειδί API. Σε σελίδες που απαιτούν SSO, το `includeUserInfo` επιστρέφει `403` για χρήστες που δεν μπορούν να δουν τη σελίδα.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** Κωδικοποιημένο σε URI JSON του αντικειμένου SSO σας. Παραλείψτε για ανώνυμους χρήστες. **/
    sso?: string
    /** Ορίστε σε 'public' για να συμπεριλάβετε τους χρήστες που αντέδρασαν. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
    /** Αριθμός ανά αναγνωριστικό αντίδρασης, π.χ. {"heart": 12, "laugh": 3}. Δεν ορίζεται όταν η σελίδα δεν έχει αντιδράσεις. **/
    counts?: Record<string, number>
    /** Τα αναγνωριστικά των αντιδράσεων που έχει προσθέσει ο χρήστης που κάνει το αίτημα. Δεν ορίζεται όταν δεν έχει προσθέσει καμία. **/
    reactedIds?: string[]
    /** Μία καταχώρηση ανά αντίδραση, πιο πρόσφατες πρώτες. Ορίζεται μόνο όταν περάσει το includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** Το αναγνωριστικό της αντίδρασης, π.χ. "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]