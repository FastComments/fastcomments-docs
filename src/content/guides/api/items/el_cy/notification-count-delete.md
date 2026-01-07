[api-resource-header-start name = 'NotificationCount'; route = 'DELETE /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή διαγράφει ένα μόνο `NotificationCount` με βάση το αναγνωριστικό χρήστη. Με SSO, το αναγνωριστικό χρήστη έχει τη μορφή `<tenant id>:<user id>`.

Αυτό θα καθαρίσει τον αριθμό μη αναγνωσμένων ειδοποιήσεων του χρήστη (το κόκκινο καμπανάκι στο widget σχολίων θα εξασθενίσει και ο αριθμός θα εξαφανιστεί).

[inline-code-attrs-start title = 'DELETE NotificationCount cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success"}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αφαίρεσης NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Αφαίρεσης NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
