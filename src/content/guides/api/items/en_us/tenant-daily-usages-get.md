[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

This route allows searching for the usage of a tenant by year, month, and day. Up to 365 objects can be returned, and the cost is 1 api credit per 10 objects.

Response objects are sorted by the date they are created (the oldest first).

[inline-code-attrs-start title = 'TenantDailyUsage Search cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Based on UTC. **/
    yearNumber?: number
    /** Zero-based. Based on UTC. **/
    monthNumber?: number
    /** One-based. Based on UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]
