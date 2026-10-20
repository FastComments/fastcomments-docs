[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Ova ruta vraća batch poslove najmodera svih tipova, najnovije prvo, 100 po stranici. Upotrijebite `type` i `status` za sužavanje popisa, i `skip` za paginaciju.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za popis BatchJobova'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za popis BatchJobova'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Samo poslovi ovog tipa, na primjer migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Koliko poslova preskočiti, za paginaciju. Nenegativni cijeli broj. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za popis BatchJobova'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    /** Najnovije prvo, najviše 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]