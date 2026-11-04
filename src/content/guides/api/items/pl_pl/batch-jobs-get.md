[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Ta trasa zwraca zadania wsadowe (batch jobs) dzierżawy każdego typu, od najnowszych, 100 na stronę. Użyj `type` i `status`, aby zawęzić listę, oraz `skip`, aby ją stronicować.

[inline-code-attrs-start title = 'Przykład cURL listy BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania listy BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Tylko zadania tego typu, na przykład migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Ile zadań pominąć, dla stronicowania. Nieujemna liczba całkowita. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi listy BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku błędu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Dołączane w przypadku błędu. **/
    reason?: string
    /** Od najnowszych, maksymalnie 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]