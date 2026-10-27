[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

此路由返回租戶的所有類型的批次工作，最新的在前，每頁最多 100 筆。使用 `type` 和 `status` 來縮小列表，使用 `skip` 進行分頁。

[inline-code-attrs-start title = 'BatchJob 列表 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob 列表 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** 僅此類型的工作，例如 migrate-pages。 **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** 要跳過多少工作，用於分頁。非負整數。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob 列表 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** 失敗時包含。 **/
    reason?: string
    /** 最新的在前，最多 100 筆。 **/
    batchJobs?: BatchJob[]
}
[inline-code-end]