[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Επιστρέφει τον αριθμό των likes σε μια σελίδα και αν ο τρέχων χρήστης την έχει κάνει like. Οι σελίδες που δεν υπάρχουν ακόμη επιστρέφουν ένα `likeCount` του `0`.

Περάστε `includeUserInfo=public` για να λάβετε επίσης τους χρήστες που έκαναν like στη σελίδα, με τη νεότερη πρώτη. Οι χρήστες με ιδιωτική δραστηριότητα προφίλ και οι ανώνυμοι χρήστες εξαιρούνται, και αναζητούνται έως 100 likes. Μόνο το `public` γίνεται αποδεκτό εδώ· το `include-private` επιστρέφει ένα `422` και είναι διαθέσιμο από [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) με κλειδί API. Σε σελίδες που απαιτούν SSO, το `includeUserInfo` επιστρέφει ένα `403` για χρήστες που δεν μπορούν να δουν τη σελίδα.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Likes Σελίδας'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Likes Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Likes Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    likeCount: number
    /** Whether the user making the request has liked the page. **/
    didLike: boolean
    /** The number of top level comments on the page. **/
    commentCount: number
    /** The id used to subscribe to live updates for this page. **/
    urlIdWS: string
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