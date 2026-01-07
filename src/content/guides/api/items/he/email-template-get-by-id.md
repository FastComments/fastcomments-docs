[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

ניתן לאחזר תבניות אימייל בודדות לפי ה-`id` התואם שלהן (לא `emailTemplateId`).

[inline-code-attrs-start title = 'דוגמת cURL לתבנית אימייל לפי מזהה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת תבנית אימייל לפי מזהה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesByIdRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת תבנית אימייל לפי מזהה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'internal' | 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found'
    /** Included on failure. **/
    reason?: string
    emailTemplate?: EmailTemplate | null
}
[inline-code-end]
