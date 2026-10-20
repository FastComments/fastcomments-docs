[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Ta pot vrne paketna opravila najemnika vseh vrst, najprej najnovejša, 100 na stran. Uporabite `type` in `status`, da zožita seznam, ter `skip` za straničenje.

[inline-code-attrs-start title = 'Primer cURL zahteve za seznam BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za seznam BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Samo opravila tega tipa, na primer migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Koliko opravil naj se preskoči, za straničenje. Ne-negativno celo število. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za seznam BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Vključeno v primeru napake. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Vključeno v primeru napake. **/
    reason?: string
    /** Najnovejše najprej, največ 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]

---