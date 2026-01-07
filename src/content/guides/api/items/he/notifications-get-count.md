[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

נתיב זה מחזיר אובייקט המכיל את מספר ההתראות תחת פרמטר `count`.

הוא איטי יותר מ-`/notification-count/` ועולה כפול בקרדיטים, אך מאפשר סינון על יותר ממדים.

אתה יכול לסנן לפי אותם פרמטרים כמו נקודת הקצה `/notifications` כמו `userId`. עם SSO, מזהה המשתמש הוא בפורמט `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'דוגמת cURL לספירת התראות שלא נקראו למשתמש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'דוגמת cURL לספירת התראות שלא נקראו למשתמש עבור עמוד ספציפי'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת ספירת התראות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'מבנה תגובת ספירת התראות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
