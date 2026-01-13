[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Esta rota permite pesquisar o uso de um tenant por ano, mês e dia. Podem ser retornados até 365 objetos, e o custo é de 1 crédito de API para cada 10 objetos.

Os objetos de resposta são ordenados pela data em que foram criados (do mais antigo para o mais recente).

[inline-code-attrs-start title = 'Exemplo cURL de pesquisa TenantDailyUsage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Com base em UTC. **/
    yearNumber?: number
    /** Baseado em zero. Com base em UTC. **/
    monthNumber?: number
    /** Baseado em um. Com base em UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Incluído em caso de falha. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]