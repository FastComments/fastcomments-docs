[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

अलग-अलग EmailTemplates को उनके संबंधित `id` (`emailTemplateId` नहीं) से प्राप्त किया जा सकता है।

[inline-code-attrs-start title = 'ID द्वारा EmailTemplate cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'ID द्वारा EmailTemplate रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesByIdRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ID द्वारा EmailTemplate रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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