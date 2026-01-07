[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Diese Route ermöglicht die Suche nach der Nutzung eines Tenants nach Jahr, Monat und Tag. Es können bis zu 365 Objekte zurückgegeben werden, und die Kosten betragen 1 API-Credit pro 10 Objekte.

Antwortobjekte sind nach ihrem Erstellungsdatum sortiert (die ältesten zuerst).

[inline-code-attrs-start title = 'TenantDailyUsage Suche cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'TenantDailyUsage Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
