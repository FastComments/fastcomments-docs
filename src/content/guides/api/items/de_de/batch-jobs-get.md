[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Diese Route gibt die Batch-Jobs des Mandanten aller Typen zurück, neueste zuerst, 100 pro Seite. Verwenden Sie `type` und `status`, um die Liste einzugrenzen, und `skip`, um durch die Seiten zu blättern.

[inline-code-attrs-start title = 'BatchJob-Liste cURL-Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob-Liste Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Nur Jobs dieses Typs, zum Beispiel migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Wie viele Jobs zu überspringen sind, für die Seitennavigation. Eine nicht-negative ganze Zahl. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob-Liste Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Bei einem Fehler enthalten. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Bei einem Fehler enthalten. **/
    reason?: string
    /** Neueste zuerst, höchstens 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]