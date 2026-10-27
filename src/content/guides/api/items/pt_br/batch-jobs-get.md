[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Esta rota retorna os jobs em lote do locatário de todos os tipos, mais recentes primeiro, 100 por página. Use `type` e `status` para filtrar a lista e `skip` para paginar.

[inline-code-attrs-start title = 'Exemplo cURL da Lista de BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Solicitação da Lista de BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Somente jobs deste tipo, por exemplo migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Quantos jobs pular, para paginação. Um inteiro não negativo. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta da Lista de BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Incluído em caso de falha. **/
    reason?: string
    /** Mais recentes primeiro, no máximo 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]