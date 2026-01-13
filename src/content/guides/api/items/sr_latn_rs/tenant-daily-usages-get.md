[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava pretragu upotrebe tenanta po godini, mesecu i danu. Može se vratiti najviše 365 objekata, a cena je 1 api credit na 10 objekata.

Objekti odgovora su sortirani po datumu kada su kreirani (najstariji prvi).

[inline-code-attrs-start title = 'Primer cURL zahteva za pretragu TenantDailyUsage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Bazirano na UTC. **/
    yearNumber?: number
    /** Indeksirano od nule. Bazirano na UTC. **/
    monthNumber?: number
    /** Indeksirano počev od 1. Bazirano na UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]