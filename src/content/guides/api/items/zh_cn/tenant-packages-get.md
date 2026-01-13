[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

此 API 使用分页，由 `skip` 查询参数提供。TenantPackages 按 `100` 条每页返回，按 `createdAt` 和 `id` 排序。

计费基于返回的 tenant packages 数量，计费为 `1 credit per 10` 个 tenant packages。

[inline-code-attrs-start title = 'TenantPackage cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 用于分页要跳过的 tenant packages 数量。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失败时包含。 **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]