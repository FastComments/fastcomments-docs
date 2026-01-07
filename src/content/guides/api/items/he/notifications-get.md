[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

נתיב זה מחזיר עד 30 אובייקטי `Notification` ממוינים לפי `createdAt`, החדש ביותר ראשון.

אתה יכול לסנן לפי `userId`. עם SSO, מזהה המשתמש הוא בפורמט `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'דוגמת cURL להתראות לא נקראות למשתמש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת התראות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'מבנה תגובת התראות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
