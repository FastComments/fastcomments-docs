[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

This route allows searching for the usage of a tenant by year, month, and day. Up to 365 objects can be returned, and the cost is 1 api credit per 10 objects.

Response objects are sorted by the date they are created (the oldest first).

[inline-code-attrs-start title = 'Приклад cURL-запиту для пошуку TenantDailyUsage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** На основі UTC. **/
    yearNumber?: number
    /** Нульова індексація. На основі UTC. **/
    monthNumber?: number
    /** Індексація з одиниці. На основі UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Включається при невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Включається при невдачі. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]