[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

Cette API utilise la pagination, fournie par le paramètre de requête `page`. Les EmailTemplates sont retournés par pages de `100`, triés par `createdAt` puis `id`.

[inline-code-attrs-start title = 'Exemple cURL de EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The page to fetch, starting with 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
