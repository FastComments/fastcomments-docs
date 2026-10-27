[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Овај рут враћа batch послове закупца свих типова, најновије прво, по 100 по страници. Користите `type` и `status` да сузите листу, и `skip` за пагинацију.

[inline-code-attrs-start title = 'Пример cURL захтева за листу BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за листу BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Само послови овог типа, на пример migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Колико послова да се прескочи, за пагинацију. Не-негативни цео број. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за листу BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају грешке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Укључено у случају грешке. **/
    reason?: string
    /** Најновије прво, највише 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]