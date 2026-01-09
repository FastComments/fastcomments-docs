[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

Појединачни EmailTemplates могу се преузети помоћу одговарајућег `id` (НЕ `emailTemplateId`).

[inline-code-attrs-start title = 'EmailTemplate по ID — пример cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate по ID — структура захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesByIdRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate по ID — структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'internal' | 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found'
    /** Укључено у случају неуспеха. **/
    reason?: string
    emailTemplate?: EmailTemplate | null
}
[inline-code-end]

---