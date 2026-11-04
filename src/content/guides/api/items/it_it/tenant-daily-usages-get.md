[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Questa rotta consente di cercare l'utilizzo di un tenant per anno, mese e giorno. È possibile restituire fino a 365 oggetti e il costo è di 1 credito API ogni 10 oggetti.

Gli oggetti della risposta sono ordinati per data di creazione (dal più vecchio al più recente).

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
    /** A base zero. Basato su UTC. **/
    monthNumber?: number
    /** A base uno. Basato su UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Incluso in caso di errore. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]