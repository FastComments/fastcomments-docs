[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

此 API 使用分页，通过查询参数 `skip` 提供。TenantUsers 以每页 `100` 条的形式返回，按 `signUpDate`、`username` 和 `id` 排序。

费用基于返回的租户用户数量计算，每返回 `10` 个租户用户消耗 `1` 积分。

[inline-code-attrs-start title = 'TenantUser cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 用于分页要跳过的租户用户数量。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失败时包含。 **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---