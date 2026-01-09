[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

此 API 返回由您的租户管理的租户。

分页由查询参数 `skip` 提供。租户以每页 `100` 个返回，按 `signUpDate` 和 `id` 排序。

费用基于返回的租户数量，按 `1 credit per 10` 租户计费。

[inline-code-attrs-start title = '租户 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

您可以在 `Tenant` 对象上定义 `meta` 参数并查询匹配的租户。例如，对于键 `someKey` 和 meta 值 `some-value`，我们可以
构造一个包含该键/值对的 JSON 对象，然后对其进行 URI 编码作为查询参数进行过滤：

[inline-code-attrs-start title = '按 meta 查询租户 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = '租户 请求 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 用于分页要跳过的租户数量。 **/
    skip?: number
    /** 按 meta 参数过滤。 **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = '租户 响应 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失败时包含。 **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]