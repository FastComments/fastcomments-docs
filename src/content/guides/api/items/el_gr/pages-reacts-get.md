[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Επιστρέφει τον αριθμό για κάθε αντίδραση σε μια σελίδα, και προαιρετικά τους χρήστες που αντέδρασαν, ως μία καταχώρηση ανά αντίδραση με το αναγνωριστικό της αντίδρασης, πιο πρόσφατες πρώτες. Αυτή είναι η έκδοση διακομιστή του [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` ελέγχει ποιοι χρήστες επιστρέφονται:

- `public` παραλείπει τους χρήστες που έχουν κάνει την δραστηριότητα του προφίλ τους ιδιωτική.
- `include-private` τους περιλαμβάνει. Χρησιμοποιήστε το μόνο όταν η λίστα δεν εμφανίζεται στο κοινό.

Οι ανώνυμοι χρήστες δεν περιλαμβάνονται ποτέ, και αναζητούνται έως 100 αντιδράσεις. Σελίδες που δεν υπάρχουν επιστρέφουν κανένα `counts` και μια κενή λίστα `users`.

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
    /** Omit to only get the counts. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Count per reaction id, for example {"heart": 12, "laugh": 3}. Not set when the page has no reactions. **/
    counts?: Record<string, number>
    /** One entry per react, most recent first. Only set when includeUserInfo is passed. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** The reaction id, for example "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Helpful Tip

Θυμηθείτε να κωδικοποιείτε URI τις τιμές όπως το `urlId`.