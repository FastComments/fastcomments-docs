[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Αυτή η διαδρομή επιστρέφει ένα αντικείμενο που περιέχει τον αριθμό ειδοποιήσεων σε μια παράμετρο `count`.

Είναι πιο αργή από την `/notification-count/` και διπλάσιου κόστους πιστώσεων, αλλά επιτρέπει φιλτράρισμα σε περισσότερες διαστάσεις.

Μπορείτε να φιλτράρετε με τις ίδιες παραμέτρους όπως το endpoint `/notifications` όπως το `userId`. Με SSO, το αναγνωριστικό χρήστη έχει τη μορφή `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Αριθμός Μη Αναγνωσμένων Ειδοποιήσεων Για Χρήστη cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Αριθμός Μη Αναγνωσμένων Ειδοποιήσεων Για Χρήστη Για Συγκεκριμένη Σελίδα cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αριθμού Ειδοποιήσεων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Filter by user. **/
    userId?: string
    /** Filter by urlId. **/
    urlId?: string
    /** Filter by source comment. **/
    fromCommentId?: string
    /** Filter by read/unread. **/
    viewed?: 'true' | 'false'
    /** Filter by type. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Αριθμού Ειδοποιήσεων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    count?: number
}
[inline-code-end]
