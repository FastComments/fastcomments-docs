[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Този маршрут връща пакетните задачи на наемателя от всички типове, най-новите първи, по 100 на страница. Използвайте `type` и `status`, за да стесните списъка, и `skip`, за да превъртате страниците.

[inline-code-attrs-start title = 'Пример за cURL за списък на BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за списък на BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Само задачи от този тип, например migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Колко задачи да се пропуснат, за странициране. Неотрицателно цяло число. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за списък на BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Включено при неуспех. **/
    reason?: string
    /** Най-новите първи, най-много 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]