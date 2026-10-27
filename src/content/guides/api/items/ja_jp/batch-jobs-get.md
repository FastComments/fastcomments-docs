[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

このエンドポイントはテナントのすべてのタイプのバッチジョブを、最新順に最大 100 件ずつページングして返します。`type` と `status` を使用してリストを絞り込み、`skip` でページングできます。

[inline-code-attrs-start title = 'BatchJob リスト cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob リスト リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** このタイプのジョブのみ、例: migrate-pages。 **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** ページングのためにスキップするジョブ数。負でない整数。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob リスト レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** 最新順、最大 100 件。 **/
    batchJobs?: BatchJob[]
}
[inline-code-end]