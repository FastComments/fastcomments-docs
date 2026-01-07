[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

כרגע אתה יכול לאחזר את כל העמודים (או עמוד בודד דרך `/by-url-id`) המשויכים לחשבון שלך. אם תרצה חיפוש מדויק יותר, [פנה אלינו](https://fastcomments.com/auth/my-account/help).

[inline-code-attrs-start title = 'דוגמת cURL לעמודים'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת עמודים'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת עמודים'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### טיפ מועיל

ה-API של `Comment` דורש `urlId`. אתה יכול לקרוא קודם ל-API של `Pages`, כדי לראות איך נראים ערכי `urlId` הזמינים לך.
