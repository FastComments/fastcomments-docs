[api-resource-header-start name = 'Page'; route = 'DELETE /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

נתיב זה מספק הסרה של עמוד בודד לפי מזהה.

שים לב שאינטראקציה עם ווידג'ט התגובות לעמוד עם אותו `urlId` פשוט תיצור מחדש את `Page` בצורה חלקה.

[inline-code-attrs-start title = 'דוגמת cURL להסרת עמוד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/pages/some-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת הסרת עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת הסרת עמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'page-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
