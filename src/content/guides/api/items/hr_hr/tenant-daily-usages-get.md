[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje pretraživanje upotrebe tenant-a po godini, mjesecu i danu. Može se vratiti do 365 objekata, a trošak je 1 API kredit na svakih 10 objekata.

Objekti odgovora sortirani su po datumu kada su kreirani (najstariji prvi).

[inline-code-attrs-start title = 'Primjer cURL pretrage TenantDailyUsage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Temeljeno na UTC. **/
    yearNumber?: number
    /** Nula-bazirano. Temeljeno na UTC. **/
    monthNumber?: number
    /** Jedno-bazirano. Temeljeno na UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]