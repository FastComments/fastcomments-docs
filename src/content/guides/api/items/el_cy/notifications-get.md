[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή επιστρέφει έως 30 αντικείμενα `Notification` ταξινομημένα κατά `createdAt`, με τα πιο πρόσφατα πρώτα.

Μπορείτε να φιλτράρετε κατά `userId`. Με SSO, το αναγνωριστικό χρήστη έχει τη μορφή `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Μη Αναγνωσμένες Ειδοποιήσεις Για Χρήστη cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Ειδοποιήσεων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginate by skipping records. **/
    skip?: number
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

[inline-code-attrs-start title = 'Δομή Απάντησης Ειδοποιήσεων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]
