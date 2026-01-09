[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Esta API usa paginação, fornecida pelo parâmetro de consulta `page`. EmailTemplates são retornados em páginas de `100`, ordenados por `createdAt` e depois por `id`.

[inline-code-attrs-start title = 'Exemplo cURL de EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** A página a ser buscada, começando em 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluído em caso de falha. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]

---