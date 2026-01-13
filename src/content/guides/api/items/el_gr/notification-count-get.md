[api-resource-header-start name = 'NotificationCount'; route = 'GET /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή επιστρέφει ένα μόνο `NotificationCount` με βάση το αναγνωριστικό χρήστη. Με SSO, το αναγνωριστικό χρήστη έχει τη μορφή `<tenant id>:<user id>`.

Αν δεν υπάρχουν μη αναγνωσμένες ειδοποιήσεις, δεν θα υπάρχει `NotificationCount` - οπότε θα λάβετε 404.

Αυτό διαφέρει από το `notifications/count` στο ότι είναι πολύ πιο γρήγορο, αλλά δεν επιτρέπει φιλτράρισμα.

[inline-code-attrs-start title = 'NotificationCount Ανά ID cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success","data":{"count":1,"createdAt":"2023-03-06T18:45:01.726Z","expireAt":"2024-03-06T01:25:01.726Z","id":"example"}}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
    data?: NotificationCount
}
[inline-code-end]
