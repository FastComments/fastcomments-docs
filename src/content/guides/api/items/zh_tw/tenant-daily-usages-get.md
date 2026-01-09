[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

此路由允許按年份、月份和日期搜尋租戶的使用情況。最多可回傳 365 個物件，費用為每 10 個物件 1 個 API 點數。

回應物件依建立日期排序（最舊的在前）。

[inline-code-attrs-start title = 'TenantDailyUsage 搜尋 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** 基於 UTC。 **/
    yearNumber?: number
    /** 從零開始。基於 UTC。 **/
    monthNumber?: number
    /** 從 1 開始。基於 UTC。 **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** 失敗時會包含。 **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]