[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

此 API 使用分頁機制，由 `skip` 查詢參數提供。TenantUsers 以每頁 `100` 筆回傳，排序依序為 `signUpDate`、`username` 與 `id`。

費用依回傳的租戶使用者數量計算，花費為 `1 credit per 10`（每 10 位租戶使用者）。

[inline-code-attrs-start title = 'TenantUser cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 分頁時要跳過的租戶使用者數量。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時會包含。 **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]