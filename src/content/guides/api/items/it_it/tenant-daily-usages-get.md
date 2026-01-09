[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

This route allows searching for the usage of a tenant by year, month, and day. Up to 365 objects can be returned, and the cost is 1 api credit per 10 objects.

Response objects are sorted by the date they are created (the oldest first).

[inline-code-attrs-start title = 'Esempio cURL di ricerca TenantDailyUsage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Basato su UTC. **/
    yearNumber?: number
    /** Indicizzato a partire da zero. Basato su UTC. **/
    monthNumber?: number
    /** Indicizzato a partire da uno. Basato su UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Incluso in caso di errore. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]