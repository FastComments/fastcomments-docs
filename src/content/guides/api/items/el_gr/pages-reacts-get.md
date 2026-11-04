[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Επιστρέφει τον αριθμό για κάθε αντίδραση σε μια σελίδα, και προαιρετικά τους χρήστες που αντέδρασαν, ως μία καταχώρηση ανά αντίδραση με το reaction id της, με τις πιο πρόσφατες πρώτες. Αυτή είναι η server-side έκδοση του [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

Το `includeUserInfo` ελέγχει ποιες χρήστες επιστρέφονται:

- Το `public` αφήνει εκτός τους χρήστες που έχουν κάνει ιδιωτική τη δραστηριότητα του προφίλ τους.
- Το `include-private` τους συμπεριλαμβάνει. Χρησιμοποιήστε το μόνο όταν η λίστα δεν εμφανίζεται στο κοινό.

Οι ανώνυμοι χρήστες δεν συμπεριλαμβάνονται ποτέ, και αναζητούνται έως 100 αντιδράσεις. Σελίδες που δεν υπάρχουν δεν επιστρέφουν `counts` και επιστρέφουν κενή λίστα `users`.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Παραλείψτε το για να λάβετε μόνο τα counts. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε αποτυχία. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Συμπεριλαμβάνεται σε αποτυχία. **/
    reason?: string
    /** Πλήθος ανά reaction id, για παράδειγμα {"heart": 12, "laugh": 3}. Δεν ορίζεται όταν η σελίδα δεν έχει αντιδράσεις. **/
    counts?: Record<string, number>
    /** Μία καταχώρηση ανά αντίδραση, με τις πιο πρόσφατες πρώτες. Ορίζεται μόνο όταν περνάει το includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** Το reaction id, για παράδειγμα "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Χρήσιμη Συμβουλή

Θυμηθείτε να κάνετε URI Encode τιμές όπως το `urlId`.