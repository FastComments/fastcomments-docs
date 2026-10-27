[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Deze route retourneert de batchtaken van de tenant van elk type, nieuwste eerst, 100 per pagina. Gebruik `type` en `status` om de lijst te filteren, en `skip` om er doorheen te pagineren.

[inline-code-attrs-start title = 'BatchJob Lijst cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob Lijst Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Alleen taken van dit type, bijvoorbeeld migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Hoeveel taken over te slaan, voor paginering. Een niet-negatief geheel getal. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob Lijst Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij falen. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Opgenomen bij falen. **/
    reason?: string
    /** Nieuwste eerst, maximaal 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]