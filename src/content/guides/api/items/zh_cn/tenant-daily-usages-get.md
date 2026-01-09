---
[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

此路由允许按年、月和日搜索租户的使用情况。最多可返回 365 个对象，费用为每 10 个对象 1 个 API 积分。

响应对象按创建日期排序（最旧的在前）。

[inline-code-attrs-start title = 'TenantDailyUsage 搜索 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** 基于 UTC。 **/
    yearNumber?: number
    /** 从 0 开始。基于 UTC。 **/
    monthNumber?: number
    /** 从 1 开始。基于 UTC。 **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** 失败时包含。 **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]

---