[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

Этот маршрут позволяет искать использование арендатора по году, месяцу и дню. Можно вернуть до 365 объектов, а стоимость — 1 апи-кредит за 10 объектов.

Объекты ответа сортируются по дате их создания (сначала самые старые).

[inline-code-attrs-start title = 'Пример cURL-запроса поиска TenantDailyUsage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** На основе UTC. **/
    yearNumber?: number
    /** Нумерация с нуля. По UTC. **/
    monthNumber?: number
    /** Нумерация с единицы. По UTC. **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа TenantDailyUsage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Указывается при ошибке. **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]

---