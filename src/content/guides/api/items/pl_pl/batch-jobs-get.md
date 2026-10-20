[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Ta trasa zwraca zadania wsadowe najemcy każdego typu, najnowsze najpierw, po 100 na stronę. Użyj `type` i `status`, aby zawęzić listę, oraz `skip`, aby stronicować wyniki.

[inline-code-attrs-start title = 'Przykład cURL listy BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania listy BatchJob; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Tylko zadania tego typu, na przykład migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Ile zadań pominąć, przy stronicowaniu. Nieujemna liczba całkowita. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi listy BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    /** Najpierw najnowsze, maksymalnie 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]