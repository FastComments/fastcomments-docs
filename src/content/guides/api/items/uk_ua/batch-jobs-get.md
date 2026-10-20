[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Цей маршрут повертає пакетні завдання орендаря всіх типів, спочатку найновіші, по 100 на сторінку. Використовуйте `type` і `status`, щоб звузити список, і `skip` для посторінкового перегляду.

[inline-code-attrs-start title = 'Приклад cURL запиту BatchJob List'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту списку BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Тільки завдання цього типу, наприклад migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Скільки завдань пропустити, для посторінкового перегляду. Невід’ємне ціле число. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді списку BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Включено у випадку помилки. **/
    reason?: string
    /** Спочатку найновіші, максимум 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]