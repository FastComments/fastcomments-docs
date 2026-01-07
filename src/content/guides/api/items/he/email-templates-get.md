[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

API זה משתמש בעימוד, המסופק על ידי פרמטר השאילתה `page`. תבניות אימייל מוחזרות בעמודים של `100`, ממוינות לפי `createdAt` ולאחר מכן `id`.

[inline-code-attrs-start title = 'דוגמת cURL לתבנית אימייל'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת תבנית אימייל'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The page to fetch, starting with 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת תבנית אימייל'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]
