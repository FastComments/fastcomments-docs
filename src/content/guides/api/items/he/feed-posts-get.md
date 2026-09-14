[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

מקבל פוסטים בתזרים, חדשים תחילה. הדפדוף מבוסס על סמן: העבר את ה-`_id` של הפוסט האחרון שקיבלת כ-`afterId` כדי לקבל את העמוד הבא.

עולה קרדיט אחד לכל עשרה פוסטים שמוחזרים, עם מינימום של קרדיט אחד.

[inline-code-attrs-start title = 'דוגמת cURL של FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** החזר פוסטים אחרי מזהה הפוסט הזה. השמט עבור העמוד הראשון. **/
    afterId?: string
    /** ברירת מחדל היא 10. מקסימום 1000. **/
    limit?: number
    /** החזר רק פוסטים עם התגים האלה. חזור על הפרמטר עבור יותר מתג אחד. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כשל. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** נכלל במקרה של כשל. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]