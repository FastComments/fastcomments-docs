[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Deze API gebruikt paginering, geleverd door de queryparameter `page`. EmailTemplates worden geretourneerd in pagina's van `100`, gesorteerd op `createdAt` en vervolgens `id`.

[inline-code-attrs-start title = 'EmailTemplate cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate Aanvraagstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** De pagina om op te halen, beginnend bij 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate Antwoordstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Opgenomen bij een fout. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]