[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Deze route maakt het mogelijk om het gebruik van een tenant te doorzoeken op jaar, maand en dag. Er kunnen maximaal 365 objecten worden geretourneerd, en de kosten zijn 1 API-credit per 10 objecten.

Response-objecten worden gesorteerd op de datum waarop ze zijn aangemaakt (de oudste eerst).

[inline-code-attrs-start title = 'TenantDailyUsage cURL-zoekvoorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Gebaseerd op UTC. **/
    yearNumber?: number
    /** Op 0 gebaseerd. Gebaseerd op UTC. **/
    monthNumber?: number
    /** Op 1 gebaseerd. Gebaseerd op UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Opgenomen bij fout. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]

---