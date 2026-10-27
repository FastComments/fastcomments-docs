[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Questo endpoint restituisce i batch job del tenant di ogni tipo, dal più recente al più vecchio, 100 per pagina. Usa `type` e `status` per restringere l'elenco e `skip` per paginarlo.

[inline-code-attrs-start title = 'Esempio cURL per l\'elenco BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta per l\'elenco BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Only jobs of this type, for example migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** How many jobs to skip, for paging. A non-negative integer. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta per l\'elenco BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Included on failure. **/
    reason?: string
    /** Newest first, at most 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]