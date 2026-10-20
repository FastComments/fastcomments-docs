[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Ova ruta vraća batch poslove zakupca svih tipova, najnovije prvo, po 100 po stranici. Koristite `type` i `status` da suzite listu, i `skip` za paginaciju.

[inline-code-attrs-start title = 'BatchJob Lista cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob Lista Struktura Zahteva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Samo poslovi ovog tipa, na primer migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Koliko poslova treba preskočiti, za paginaciju. Ne-negativni ceo broj. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob Lista Struktura Odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Najnovije prvo, najviše 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]

---