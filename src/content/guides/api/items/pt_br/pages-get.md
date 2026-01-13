[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Atualmente você pode apenas buscar todas as páginas (ou uma única página via `/by-url-id`) associadas à sua conta. Se desejar uma busca mais refinada, [entre em contato conosco](https://fastcomments.com/auth/my-account/help). 

[inline-code-attrs-start title = 'Exemplo cURL de Páginas'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de Páginas'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Páginas'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluído em caso de falha. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Dica Útil

A API `Comment` requer um `urlId`. Você pode chamar primeiro a API `Pages`, para ver como são os valores de `urlId` disponíveis para você.