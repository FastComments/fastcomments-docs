[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Bireysel EmailTemplates, karşılık gelen `id` ile alınabilir (NOT `emailTemplateId`).

[inline-code-attrs-start title = 'ID ile EmailTemplate cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'ID ile EmailTemplate İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesByIdRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ID ile EmailTemplate Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'internal' | 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    emailTemplate?: EmailTemplate | null
}
[inline-code-end]

---