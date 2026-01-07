[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Diese Route gibt bis zu 30 `Notification`-Objekte zurück, sortiert nach `createdAt`, neueste zuerst.

Sie können nach `userId` filtern. Bei SSO hat die Benutzer-ID das Format `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Ungelesene Benachrichtigungen für Benutzer cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Benachrichtigungen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Benachrichtigungen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
