[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at søge efter forbruget for en tenant efter år, måned og dag. Op til 365 objekter kan returneres, og omkostningen er 1 api-kredit pr. 10 objekter.

Svarobjekter sorteres efter datoen de er oprettet (de ældste først).

[inline-code-attrs-start title = 'TenantDailyUsage Søgning cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'TenantDailyUsage Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
