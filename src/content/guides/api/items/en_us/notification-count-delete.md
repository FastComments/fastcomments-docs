[api-resource-header-start name = 'NotificationCount'; route = 'DELETE /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

This route deletes a single `NotificationCount` by user id. With SSO, the user id is in the format `<tenant id>:<user id>`.

This will clear the user's unread notification count (the red bell in the comment widget will fade out and the count will go away).

[inline-code-attrs-start title = 'DELETE NotificationCount cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success"}
[inline-code-end]

[inline-code-attrs-start title = 'NotificationCount Removal Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'NotificationCount Removal Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
