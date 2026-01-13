[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Ta trasa umożliwia wyszukiwanie użycia tenantów według roku, miesiąca i dnia. Może zostać zwróconych do 365 obiektów, a koszt to 1 kredyt API za każde 10 obiektów.

Obiekty odpowiedzi są sortowane według daty ich utworzenia (najpierw najstarsze).

[inline-code-attrs-start title = 'Przykład żądania cURL wyszukiwania TenantDailyUsage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** Oparte na UTC. **/
    yearNumber?: number
    /** Indeksowane od zera. Oparte na UTC. **/
    monthNumber?: number
    /** Indeksowane od jednego. Oparte na UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Dołączane w przypadku niepowodzenia. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]

---