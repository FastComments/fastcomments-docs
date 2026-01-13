[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava pretragu upotrebe tenanta po godini, mjesecu i danu. Može se vratiti do 365 objekata, a cijena je 1 API kredit na svakih 10 objekata.

Objekti u odgovoru su sortirani po datumu kreiranja (najstariji prvi).

[inline-code-attrs-start title = 'TenantDailyUsage Pretraga cURL Primjer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage Struktura zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Bazirano na UTC. **/
    yearNumber?: number
    /** Indeksirano od 0. Bazirano na UTC. **/
    monthNumber?: number
    /** Indeksirano od 1. Bazirano na UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage Struktura odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]

---