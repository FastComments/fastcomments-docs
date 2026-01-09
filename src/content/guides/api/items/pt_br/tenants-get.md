[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Esta API retorna tenants que são gerenciados pelo seu tenant.

A paginação é fornecida pelo parâmetro de consulta `skip`. Os tenants são retornados em páginas de `100`, ordenados por `signUpDate`, e `id`.

O custo é baseado no número de tenants retornados, custando `1 credit per 10` tenants retornados.

[inline-code-attrs-start title = 'Exemplo cURL de Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Você pode definir parâmetros `meta` nos objetos `Tenant` e consultar tenants correspondentes. Por exemplo, para a chave `someKey` e o valor meta `some-value`, podemos
construir um objeto JSON com esse par chave/valor e então codificá-lo em URI como um parâmetro de consulta para filtrar:

[inline-code-attrs-start title = 'Exemplo cURL de Consulta de Tenant por Meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** O número de tenants a serem ignorados para paginação. **/
    skip?: number
    /** Filtrar pelos parâmetros meta. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluído em caso de falha. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]