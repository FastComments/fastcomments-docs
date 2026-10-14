[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Επιστρέφει τον αριθμό των likes σε μια σελίδα, και προαιρετικά τους χρήστες που την έδωσαν like, με τους πιο πρόσφατους πρώτα. Αυτή είναι η έκδοση διακομιστή του [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` ελέγχει ποιοι χρήστες επιστρέφονται:

- `public` αφήνει έξω τους χρήστες που έχουν κάνει την δραστηριότητα του προφίλ τους ιδιωτική.
- `include-private` τους περιλαμβάνει. Χρησιμοποιήστε το μόνο όταν η λίστα δεν εμφανίζεται στο κοινό.

Οι ανώνυμοι χρήστες δεν περιλαμβάνονται ποτέ, και αναζητούνται έως 100 likes. Σελίδες που δεν υπάρχουν επιστρέφουν ένα `likeCount` ίσο με `0` και μια κενή λίστα `users`.

[inline-code-attrs-start title = 'Παράδειγμα cURL Page Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    likeCount?: number
    /** Most recent first. Only set when includeUserInfo is passed. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Χρήσιμη Συμβουλή

Θυμηθείτε να κωδικοποιείτε URI τις τιμές όπως το `urlId`.

---