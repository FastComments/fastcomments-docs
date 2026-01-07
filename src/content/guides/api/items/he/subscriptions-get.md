[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

נתיב זה מחזיר עד 30 אובייקטי `Subscription` ממוינים לפי `createdAt`, החדש ביותר ראשון.

אתה יכול לסנן לפי `userId`. עם SSO, מזהה המשתמש הוא בפורמט `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'דוגמת cURL למנויים למשתמש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת מנויים'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginate by skipping records. **/
    skip?: number
    /** Filter by user. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת מנויים'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]
