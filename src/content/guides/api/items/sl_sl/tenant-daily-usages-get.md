[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Ta pot omogoča iskanje uporabe najemnika po letu, mesecu in dnevu. Vrne se lahko do 365 objektov, cena pa je 1 api credit na 10 objektov.

Objekti v odgovoru so razvrščeni po datumu ustvarjanja (najstarejši prvi).

[inline-code-attrs-start title = 'Primer cURL poizvedbe TenantDailyUsage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Glede na UTC. **/
    yearNumber?: number
    /** Indeksirano od 0. Glede na UTC. **/
    monthNumber?: number
    /** Indeksirano od 1. Glede na UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Vključeno ob napaki. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]

---