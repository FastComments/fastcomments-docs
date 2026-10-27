[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

이 라우트는 테넌트의 모든 유형의 배치 작업을 최신 순으로 반환하며, 페이지당 최대 100개를 제공합니다. 목록을 좁히려면 `type` 및 `status`를 사용하고, 페이지를 이동하려면 `skip`을 사용하세요.

[inline-code-attrs-start title = 'BatchJob 목록 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob 목록 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** 이 유형의 작업만 포함합니다. 예: migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** 페이지 매김을 위해 건너뛸 작업 수. 음수가 아닌 정수. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob 목록 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 최신 순으로, 최대 100개. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]