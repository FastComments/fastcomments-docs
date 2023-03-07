[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

This route returns up to 30 `Notification` objects sorted by `createdAt`, newest first.

You can filter by `userId`. With SSO, the user id is in the format `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Unread Notifications For User cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Notifications Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginate by skipping records. **/
    skip?: number
    /** Filter by user. **/
    userId?: string
    /** Filter by source comment. **/
    fromCommentId?: string
    /** Filter by read/unread. **/
    viewed?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Notifications Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]
